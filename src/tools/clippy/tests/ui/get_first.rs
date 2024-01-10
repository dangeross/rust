#![warn(clippy::get_first)]
#![allow(clippy::useless_vec)]
use std::collections::{BTreeMap, HashMap, VecDeque};

struct Bar {
    arr: [u32; 3],
}

impl Bar {
    fn get(&self, pos: usize) -> Option<&u32> {
        self.arr.get(pos)
    }
}

fn main() {
    let x = vec![2, 3, 5];
    let _ = x.get(0);
    //~^ ERROR: accessing first element with `x.get(0)`
    let _ = x.get(1);
    let _ = x[0];

    let y = [2, 3, 5];
    let _ = y.get(0);
    //~^ ERROR: accessing first element with `y.get(0)`
    let _ = y.get(1);
    let _ = y[0];

    let z = &[2, 3, 5];
    let _ = z.get(0);
    //~^ ERROR: accessing first element with `z.get(0)`
    let _ = z.get(1);
    let _ = z[0];

    let vecdeque: VecDeque<_> = x.iter().cloned().collect();
    let _ = vecdeque.get(0);
    //~^ ERROR: accessing first element with `vecdeque.get(0)`
    let _ = vecdeque.get(1);

    let hashmap: HashMap<u8, char> = HashMap::from_iter(vec![(0, 'a'), (1, 'b')]);
    let btreemap: BTreeMap<u8, char> = BTreeMap::from_iter(vec![(0, 'a'), (1, 'b')]);
    let _ = hashmap.get(&0); // Do not lint, because HashMap is not slice.
    let _ = btreemap.get(&0); // Do not lint, because BTreeMap is not slice.

    let bar = Bar { arr: [0, 1, 2] };
    let _ = bar.get(0); // Do not lint, because Bar is struct.

    let non_primitives = [vec![1, 2], vec![3, 4]];
    let _ = non_primitives.get(0);
    //~^ ERROR: accessing first element with `non_primitives.get(0)`
}
