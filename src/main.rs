use itertools::Itertools;
use std::fs;
mod vm;

fn read_file(path: &str) -> Vec<u16> {
  let buf = fs::read(path).unwrap();
  return buf.iter()
    .tuples()
    .map(|(&a,&b)| (b as u16) << 8 | a as u16)
    .collect::<Vec<_>>();
}

fn main() {
    let memory = read_file("challenge.bin");
    let vm_run = vm::Vm::new(memory);
    vm_run.execute();
}
