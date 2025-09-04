use std/assert

for d in 0..366000 {
  # let $d = $d - ($d // 36525)
  # print ($d // 36525)
  # let a = match ($d mod 1461) {
  #   ..365 => 0, # 366 days with day 0 (leap year)
  #   ..730 => 1,
  #   ..1095 => 2,
  #   ..1460 => 3,
  # };
  # let b = 4 * ($d // 1461);
  # let y = $a + $b;

  let SECONDS_PER_DAY = 86400;
  # let day = $epoch_sec // $SECONDS_PER_DAY;
  let day = $d;
   
  let DAYS_PER_YEAR = 365;
  let DAYS_PER_4_YEARS = $DAYS_PER_YEAR * 4 + 1; # 4 Years and a extra day
  let DAYS_PER_100_YEARS = $DAYS_PER_4_YEARS * 25 - 1; # 25 times 4 years but this time without the extra leap year day
  let DAYS_PER_400_YEARS = $DAYS_PER_100_YEARS * 4 + 1; # Exception to the 100 Year rule we add the leap year back

  let n400 = $day // $DAYS_PER_400_YEARS;
  let day = $day mod $DAYS_PER_400_YEARS;

  let n100 = [($day // $DAYS_PER_100_YEARS), 3] | math min;
  let day = $day - $n100 * $DAYS_PER_100_YEARS;

  let n4 = $day // $DAYS_PER_4_YEARS;
  let day = $day mod $DAYS_PER_4_YEARS;

  let n1 = [($day // $DAYS_PER_YEAR)] | math min;
  let day = $day - $n1 * $DAYS_PER_YEAR;

  let y = 400 * $n400 + 100 * $n100 + 4 * $n4 + $n1;
  print $"($d) ($n400) ($n100) ($n4) ($n1)"
  # Adding 2 years (730 days) to synchronize leap years with 1970, also account in subtraction for those 2 years together with 1970
  let test_y = (($d + 730) * 86400 | into datetime -f "%s" | into record | get year) - 1972
  assert ($y == $test_y) --error-label {
    text: $"day: ($d) is year ($test_y): ($y), (($d + 730) * 86400 | into datetime -f "%s" | into record)"
  }
  print $"day: ($d) is year ($test_y): ($y)"
}

print "Calculation ok! :)"
