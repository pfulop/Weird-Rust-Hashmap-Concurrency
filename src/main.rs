use rand::Rng;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Point {
    property: u32,
}

impl Point {
    fn new(property: u32) -> Arc<Mutex<Point>> {
        let point = Point { property: property };
        let mutex = Mutex::new(point);
        Arc::new(mutex)
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let points = vec![
        Point::new(rng.gen::<u32>()),
        Point::new(rng.gen::<u32>()),
        Point::new(rng.gen::<u32>()),
    ];

    let mut points_map = HashMap::new();

    for point in &points {
        {
            points_map.insert(point.lock().unwrap().property, point);
        }
    }

    println!("{:?}", points_map);

    let mut children = vec![];
    {
        points_map.iter_mut().for_each(|(k, v)| {
            let k_cloned = k.clone();
            let v_cloned = v.clone();
            children.push(thread::spawn(move || {
                let mut point = v_cloned.lock().unwrap();
                point.property = point.property + k_cloned / 1000;
            }));
        });
    }
    for child in children {
        child.join().unwrap();
    }
    println!("{:?}", points_map);
}
