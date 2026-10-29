struct Laptop {
    brand: String,
    price: f64,
}

impl Laptop {
    fn new(brand: &str, price: f64) -> Laptop {
        Laptop {
            brand: String::from(brand),
            price: price,
        }
    }

    // &self borrows the laptop, it does not take ownership
    fn cost(&self, qty: f64) -> f64 {
        self.price * qty
    }
}

// &Laptop borrows, so the caller keeps ownership
fn show(laptop: &Laptop, cost: f64) {
    println!("{} (3 laptops): N{}", laptop.brand, cost);
}

fn main() {
    let hp = Laptop::new("HP", 650000.0);
    let ibm = Laptop::new("IBM", 755000.0);
    let toshiba = Laptop::new("Toshiba", 550000.0);
    let dell = Laptop::new("Dell", 850000.0);

    let hp_cost = hp.cost(3.0);
    let ibm_cost = ibm.cost(3.0);
    let toshiba_cost = toshiba.cost(3.0);
    let dell_cost = dell.cost(3.0);

    show(&hp, hp_cost);
    show(&ibm, ibm_cost);
    show(&toshiba, toshiba_cost);
    show(&dell, dell_cost);

    let total = hp_cost + ibm_cost + toshiba_cost + dell_cost;
    println!("Total cost: N{}", total);
}