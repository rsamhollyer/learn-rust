/*
* A type that contains multiple pieces of data.
*  - All or nothing -- it cannot have some pieces of data and not others
* Each piece of data is called a "field"
* Makes working with data easier
*  - Similar data can be grouped together
*/

struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

struct GroceryItem {
    stock: i32,
    price: f64, // Also known as a double,
}

fn main() {
    // let box1: ShippingBox = ShippingBox {
    //     depth: 10,
    //     width: 20,
    //     height: 30,
    // };

    // let tall: i32 = box1.height;

    // println!("the box is {:?} units tall", tall);

    let cereal: GroceryItem = GroceryItem {
        stock: 10,
        price: 2.99,
    };

    println!("The cereal is {:?}", cereal.price);
    println!("The cereal has {:?} item(s) in stock", cereal.stock);
}
