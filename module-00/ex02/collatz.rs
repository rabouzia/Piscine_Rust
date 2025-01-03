fn collatz(mut start: u32){
	println!("{}",start);
	while start != 1{
		if start % 2 == 0
		{start /= 2;}
		else
			{start *= 3;
			start += 1;}
		println!("{}",start);
	}
}
fn collatz_recursive(mut start: u32){
	println!("{}",start);
	if start != 1{
		if start % 2 == 0
		{collatz_recursive(start / 2);}
		else
			{collatz_recursive(start * 3 + 1);}
	}
}

fn main(){
	collatz_recursive(3);
}