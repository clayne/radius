use radius;

use radius::radius::{Radius, RadiusOption};
use radius::value::Value;

fn main() {
    let options = vec!(RadiusOption::Debug(true));
    let mut radius = Radius::new_with_options("tests/r100", options);
    let mut state = radius.call_state(0x004006fd);
    let bv = state.bv("flag", 12*8);
    let addr: u64 = 0x100000;
    state.memory.write_value(addr, Value::Symbolic(bv.clone()), 12);
    state.registers.set("rdi", Value::Concrete(addr));

    radius.breakpoint(0x004007a1);
    radius.avoid(vec!(0x00400790));
    let mut new_state = radius.run(Some(state), 1).unwrap();
    let flag = new_state.evaluate_string(&bv).unwrap();
    println!("FLAG: {}", flag);
    assert_eq!(flag, "Code_Talkers");
}