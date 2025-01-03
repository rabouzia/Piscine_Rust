

fn min (a: i32, b: i32) -> i32{
	if a < b
		{a}
	else
		{b}
}

fn main()
{
	let result = min(332, 2323);
	println!("min is: {}", result);
}