fn print_bytes(s: &str){
	for byte in s.as_bytes()
	{println!("{}", byte)}
}

fn main(){
	print_bytes("Deja vu\n");
}
