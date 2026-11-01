mod mymod;

use crate::mymod::ticket::Ticket;

fn main() {
    let ticket = Ticket{a:1,b:2};
    println!("{}{}", ticket.a, ticket.b);
}
