use std::cmp::min;
use std::collections::HashMap;
use std::hash::Hash;
use std::println;

/// Represents product info
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(unused)]
pub enum Product {
    Lolypop,
    Chips,
    Water,
    Soda,
}

#[allow(unused)]
impl Product {
    /// Price of product
    fn price(&self) -> u64 {
        match self {
            Product::Lolypop => 12,
            Product::Chips => 45,
            Product::Water => 52,
            Product::Soda => 63,
        }
    }
}

/// Supported coins nominals
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(unused)]
pub enum Coin {
    N1,
    N2,
    N5,
    N10,
    N20,
    N50,
}

impl Coin {
    /// Useful for calculate change
    fn big_to_small() -> impl Iterator<Item = Coin> {
        vec![
            Coin::N50,
            Coin::N20,
            Coin::N10,
            Coin::N5,
            Coin::N2,
            Coin::N1,
        ]
        .into_iter()
    }
}

impl From<&Coin> for u64 {
    fn from(value: &Coin) -> Self {
        match value {
            Coin::N1 => 1,
            Coin::N2 => 2,
            Coin::N5 => 5,
            Coin::N10 => 10,
            Coin::N20 => 20,
            Coin::N50 => 50,
        }
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct VendingMachine<State> {
    /// Products count
    products: HashMap<Product, u64>,
    /// Availables machine change balance
    change_balance: HashMap<Coin, u64>,
    /// Available user balance
    user_balance: HashMap<Coin, u64>,
    /// Current machine state
    state: State,
}

/// Vending machine states, represents different methods
pub struct MaintenanceState;
pub struct UserServiceState;

/// VendingMachine builder
impl VendingMachine<MaintenanceState> {
    pub fn builder() -> Self {
        VendingMachine {
            products: HashMap::new(),
            change_balance: HashMap::new(),
            user_balance: HashMap::new(),
            state: MaintenanceState,
        }
    }
    pub fn add_product(mut self, product: Product) -> Self {
        *self.products.entry(product).or_insert(0) += 1;
        self
    }

    pub fn add_products<T: IntoIterator<Item = Product>>(mut self, products: T) -> Self {
        for product in products {
            self = self.add_product(product)
        }
        self
    }

    pub fn add_coin(mut self, coin: Coin) -> Self {
        *self.change_balance.entry(coin).or_insert(0) += 1;
        self
    }

    pub fn add_coins<T: IntoIterator<Item = Coin>>(mut self, coins: T) -> Self {
        for coin in coins {
            self = self.add_coin(coin)
        }
        self
    }

    pub fn build(self) -> VendingMachine<UserServiceState> {
        VendingMachine {
            products: self.products,
            change_balance: self.change_balance,
            user_balance: self.user_balance,
            state: UserServiceState,
        }
    }
}


/// Represents purchase status
#[derive(Debug)]
#[allow(unused)]
enum PurchaseStatus {
    Success {
        product: Product,
        change: HashMap<Coin, u64>,
    },
    NotEnoughtBalance,
    NotEnoughtProduct,
    CannotGiveChange,
}

/// Simple user interface
impl VendingMachine<UserServiceState> {
    fn insert_coin(&mut self, coin: Coin) {
        *self.user_balance.entry(coin.clone()).or_insert(0) += 1;
        *self.change_balance.entry(coin).or_insert(0) += 1;
    }

    fn insert_coins<T: IntoIterator<Item = Coin>>(&mut self, coins: T) {
        for coin in coins {
            self.insert_coin(coin)
        }
    }

    fn purchase(&mut self, product: Product) -> PurchaseStatus {
        if *self.products.get(&product).unwrap_or(&0) == 0 {
            return PurchaseStatus::NotEnoughtProduct;
        }

        let user_balance = self
            .user_balance
            .iter()
            .map(|(k, v)| u64::from(k) * v)
            .sum::<u64>();
        if user_balance < product.price() {
            return PurchaseStatus::NotEnoughtBalance;
        }

        let mut change_value = user_balance - product.price();
        let mut change = HashMap::new();
        for coin in Coin::big_to_small() {
            let coins = min(
                change_value / u64::from(&coin),
                *self.change_balance.get(&coin).unwrap_or(&0),
            );
            *change.entry(coin.clone()).or_insert(0) += coins;
            change_value -= coins * u64::from(&coin);
        }

        if change_value != 0 {
            return PurchaseStatus::CannotGiveChange;
        }
        for (coin, v) in change.iter() {
            *self.change_balance.entry(coin.clone()).or_insert(0) -= v;
        }

        self.user_balance = HashMap::new();

        PurchaseStatus::Success { product, change }
    }

    fn get_balance(&self) -> &HashMap<Coin, u64> {
        &self.user_balance
    }
}

fn main() {
    let mut vm = VendingMachine::builder()
        .add_products(vec![Product::Lolypop; 5])
        .add_products(vec![Product::Chips; 5])
        .add_products(vec![Product::Water; 5])
        .add_products(vec![Product::Water; 5])
        .add_coins(vec![Coin::N50; 5])
        .add_coins(vec![Coin::N20; 5])
        .add_coins(vec![Coin::N10; 5])
        .add_coins(vec![Coin::N5; 5])
        .add_coins(vec![Coin::N2; 5])
        .add_coins(vec![Coin::N1; 5])
        .build();

    vm.insert_coin(Coin::N50);
    vm.insert_coins(vec![Coin::N50; 1]);
    println!("{:?}", vm.purchase(Product::Water));

    vm.insert_coins(vec![Coin::N20; 1]);
    println!("{:?}", vm.purchase(Product::Lolypop));

    vm.insert_coin(Coin::N50);
    vm.insert_coins(vec![Coin::N50; 1]);
    println!("{:?}", vm.purchase(Product::Water));

    vm.insert_coin(Coin::N50);
    vm.insert_coins(vec![Coin::N50; 1]);
    println!("{:?}", vm.purchase(Product::Water));

    vm.insert_coin(Coin::N50);
    vm.insert_coins(vec![Coin::N50; 1]);
    println!("{:?}", vm.purchase(Product::Water));

    vm.insert_coin(Coin::N50);
    vm.insert_coins(vec![Coin::N20; 1]);
    println!("Balance: {:?}", vm.get_balance());
    println!("{:?}", vm.purchase(Product::Water));
}
