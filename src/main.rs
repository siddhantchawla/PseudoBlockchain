mod models;
fn main() {
   let difficulty = 4;
   let mut blockchain = models::blockchain::Blockchain::new(difficulty);
   models::blockchain::Blockchain::add_block(&mut blockchain);
   models::blockchain::Blockchain::add_block(&mut blockchain);
   models::blockchain::Blockchain::add_block(&mut blockchain);
   models::blockchain::Blockchain::add_block(&mut blockchain);
   models::blockchain::Blockchain::add_block(&mut blockchain);
}