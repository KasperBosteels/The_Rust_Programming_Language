fn main() {
   let mut index = 1; //is usize type
   let mut f_numbers:[u32;20] = Default::default(); //used to generate an empty array
   f_numbers[0] = 0;
   f_numbers[1] = 1; //using index here would change it into u32 and cannot be used as index
   while index < 19 {
       index+=1;
       f_numbers[index] = f_numbers[index-1] + f_numbers[index-2];
       println!("{}", f_numbers[index]);
   }
}