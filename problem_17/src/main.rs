#[derive(Debug)]
enum NumberWord {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    Thirty,
    Forty,
    Fifty,
    Sixty,
    Seventy,
    Eighty,
    Ninety,
    Hundred,
    Thousand, // About as high we go in the problem
    And,
}

impl From<u32> for NumberWord {
    fn from(value: u32) -> Self {
        Self::from(value.to_string().as_str())
    }
}

impl From<&str> for NumberWord {
    fn from(value: &str) -> Self {
        match value {
            "0" => NumberWord::Zero,
            "1" => NumberWord::One,
            "2" => NumberWord::Two,
            "3" => NumberWord::Three,
            "4" => NumberWord::Four,
            "5" => NumberWord::Five,
            "6" => NumberWord::Six,
            "7" => NumberWord::Seven,
            "8" => NumberWord::Eight,
            "9" => NumberWord::Nine,
            "10" => NumberWord::Ten,
            "11" => NumberWord::Eleven,
            "12" => NumberWord::Twelve,
            "13" => NumberWord::Thirteen,
            "14" => NumberWord::Fourteen,
            "15" => NumberWord::Fifteen,
            "16" => NumberWord::Sixteen,
            "17" => NumberWord::Seventeen,
            "18" => NumberWord::Eighteen,
            "19" => NumberWord::Nineteen,
            "20" => NumberWord::Twenty,
            "30" => NumberWord::Thirty,
            "40" => NumberWord::Forty,
            "50" => NumberWord::Fifty,
            "60" => NumberWord::Sixty,
            "70" => NumberWord::Seventy,
            "80" => NumberWord::Eighty,
            "90" => NumberWord::Ninety,
            "100" => NumberWord::Hundred,
            "1000" => NumberWord::Thousand,
            "and" => NumberWord::And,
            _ => panic!("{value}"),
        }
    }
}

impl NumberWord {
    fn wordify_cipher_1(n: u32) -> Vec<NumberWord> {
        vec![Self::from(n)]
    }

    fn wordify_cipher_2(n: u32) -> Vec<NumberWord> {
        let mut v = Vec::new();
        if n < 20 {
            v.push(Self::from(n));
        } else {
            let r = n % 10;
            v.push(Self::from(n - r));
            if r != 0 {
                // Omit zero for twenty, thirty and so on
                v.extend(Self::wordify_cipher_1(r));
            }
        }
        v
    }

    fn wordify_cipher_3(n: u32) -> Vec<NumberWord> {
        let mut v = Vec::new();
        let r = n % 100;
        let q = n / 100;
        v.push(Self::from(q));
        v.push(Self::Hundred);
        if r != 0 {
            // if r > 10 {
            v.push(Self::And);
            // }
            v.extend(Self::wordify_cipher_2(r));
        }
        v
    }

    fn wordify_cipher_4(n: u32) -> Vec<NumberWord> {
        let mut v = Vec::new();
        let r = n % 1000;
        let q = n / 1000;
        v.push(Self::from(q));
        v.push(Self::Thousand);
        if r != 0 {
            v.push(Self::And);
            v.extend(Self::wordify_cipher_3(r));
        }
        v
    }

    pub fn wordify(number: u32) -> Vec<NumberWord> {
        let cipher_cnt = number.to_string().len();
        match cipher_cnt {
            1 => Self::wordify_cipher_1(number),
            2 => Self::wordify_cipher_2(number),
            3 => Self::wordify_cipher_3(number),
            4 => Self::wordify_cipher_4(number),
            _ => panic!("{} Unsupported cipher size", cipher_cnt),
        }
    }

    pub fn get_letter_count(&self) -> u32 {
        match self {
            NumberWord::Zero => 4,
            NumberWord::One => 3,
            NumberWord::Two => 3,
            NumberWord::Three => 5,
            NumberWord::Four => 4,
            NumberWord::Five => 4,
            NumberWord::Six => 3,
            NumberWord::Seven => 5,
            NumberWord::Eight => 5,
            NumberWord::Nine => 4,
            NumberWord::Ten => 3,
            NumberWord::Eleven => 6,
            NumberWord::Twelve => 6,
            NumberWord::Thirteen => 8,
            NumberWord::Fourteen => 8,
            NumberWord::Fifteen => 7,
            NumberWord::Sixteen => 7,
            NumberWord::Seventeen => 9,
            NumberWord::Eighteen => 8,
            NumberWord::Nineteen => 8,
            NumberWord::Twenty => 6,
            NumberWord::Thirty => 6,
            NumberWord::Forty => 5,
            NumberWord::Fifty => 5,
            NumberWord::Sixty => 5,
            NumberWord::Seventy => 7,
            NumberWord::Eighty => 6,
            NumberWord::Ninety => 6,
            NumberWord::Hundred => 7,
            NumberWord::Thousand => 8,
            NumberWord::And => 3,
        }
    }

    pub fn sum_vec(v: Vec<Self>) -> u32 {
        v.iter().map(|nw| nw.get_letter_count()).sum::<u32>()
    }
}

fn main() {
    dbg!(NumberWord::wordify(342));
    assert!(NumberWord::sum_vec(NumberWord::wordify(342)) == 23);

    dbg!(NumberWord::wordify(115));
    assert!(NumberWord::sum_vec(NumberWord::wordify(115)) == 20);

    let mut sum = 0;
    for n in 1..=1000 {
        sum += NumberWord::sum_vec(NumberWord::wordify(n));
    }
    dbg!(sum);
}
