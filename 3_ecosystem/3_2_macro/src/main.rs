use step_3_2::btreemap;

mod btreemap_declarative {
    macro_rules! btreemap {
        ( $(($key:expr , $value: expr)),* ) => {
            {
                use std::collections::BTreeMap;
                let mut temp = BTreeMap::new();
                $(
                    temp.insert($key, $value);
                )*
                temp
            }
        };
        ( $($key:expr => $value: expr),* ) => {
            {
                use std::collections::BTreeMap;
                let mut temp = BTreeMap::new();
                $(
                    temp.insert($key, $value);
                )*
                temp
            }
        };
    }
    pub(crate) use btreemap;
}

fn main() {
    let bm = btreemap_declarative::btreemap![(1, 2), (3, 4)];
    println!("{:?}", bm);
    let bm = btreemap_declarative::btreemap![1 => 2, 3 => 4];
    println!("{:?}", bm);

    let bm = btreemap![1 = 2, 3 = 4];
    println!("{:?}", bm);
}
