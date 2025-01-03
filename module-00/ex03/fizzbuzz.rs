fn fizzbuzz()
{
	let mut  n = 1;
	while n <= 101
	{
		if n % 3 == 0 && n % 5 == 0
			{ println!("fizzbuzz"); }
		else if n % 11 == 3
			{ println!("FIZZ"); }
	
		else if n % 11 == 5
			{ println!("BUZZ"); }
		
		else if n % 5 == 0
			{ println!("buzz"); }
		
		else if n % 3 == 0
			{ println!("fizz"); }

		else
			{ println!("{}", n); }

		n+=1;
	}
}

fn main ()
{
	fizzbuzz();
}