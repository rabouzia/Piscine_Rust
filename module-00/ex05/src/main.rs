std::{assert, assert_eq, assert_ne} std::panic std::{print, println};



fn is_leap_year(year: u32) ->bool
{
	if year % 4 == 0
	{
		if year % 100 == 0
		{
			if year % 400 == 0
				{ return true;}
			else
				{ return false;}	
		}
		else
			{ return true;}
	}
	else
		{ return false;}
}

fn num_days_year(year: u32) -> u32
{
	if is_leap_year(year)
		{ return 366;}
	else
		{ return 365;}
}




fn main ()
{

}test
