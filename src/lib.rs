pub struct BigNumber {
    pub bits: Vec<u8>,
}

impl BigNumber {
    pub fn from_dec(dec_str: &str) -> Self {
        let mut dec: Vec<u8> = dec_str.trim().as_bytes().to_vec();
        let mut bits: Vec<u8> = Vec::new();

        loop {
            let rem = divide_by_two(&mut dec);
            bits.push(rem);

            if dec.is_empty() || all_zero(&dec) {
                break;
            }
        }
        
        Self { bits }
    }

    pub fn to_dec(&self) -> String {
        let mut dec = vec![0];
    
        let mut power_of_two = vec![vec![1]];
        for i in 1..=self.bits.len() {
            power_of_two.push(self._add_dec(&power_of_two[i - 1], &power_of_two[i - 1]));
        }
    
        for i in 0..self.bits.len() {
            if self.bits[i] == 1 {
                dec = self._add_dec(&dec, &power_of_two[i]);
            }
        }
    
        dec.iter().map(|&d| (d + b'0') as char).collect()
    }

    pub fn to_bits(&self) -> String {
        self._reverse_bits().iter().map(|&v| v.to_string()).collect()
    }

    pub fn add(&self, num: BigNumber) -> Self {
        let bits: Vec<u8> = self._add(self.bits.clone(), num.bits);
       
        Self { bits }
    }

    pub fn sub(&self, num: BigNumber) -> Self {
        let mut bits: Vec<u8> = Vec::new();
        let iter_len = if self.bits.len() > num.bits.len() {self.bits.len()} else {num.bits.len()};

        let mut remainder: u8 = 0;
        for n in 0..iter_len {
            let bit1 = self.bits.get(n).unwrap_or(&0);
            let bit2 = num.bits.get(n).unwrap_or(&0);

            let bit1_original = *bit1;

            let bit1 = if remainder > 0 {remainder - bit1_original} else {bit1_original};
            let bit2 = *bit2;


            if bit1 == 1 && bit2 == 0 {
                bits.push(1);
            } else if bit1 == 1 && bit2 == 1 {
                bits.push(0);
            } else if bit1 == 0 && bit2 == 0 {
                bits.push(0);
            } else if bit1 == 0 && bit2 == 1 {
                bits.push(1);
            }

            if bit1_original == 0 && (bit2 == 1 || remainder == 1) || bit1 < bit2 {
                remainder = 1;
            } else {
                remainder = 0;
            }
        }

        Self { bits }
    }

    pub fn mul(&self, num: BigNumber) -> Self {
        let main_bits = if self.bits.len() >= num.bits.len() {self.bits.clone()} else {num.bits.clone()};
        let multipliers = if self.bits.len() < num.bits.len() {self.bits.clone()} else {num.bits.clone()};

        let mut bits: Vec<u8>;
        if multipliers[0] == 0 {
            bits = vec![0; main_bits.len()];
        } else {
            bits = main_bits.clone();
        }

        for n in 1..multipliers.len() {
            let multiplier = multipliers.get(n).unwrap_or(&0);
            let mut add_bits: Vec<u8>;

            if *multiplier == 0 {
                add_bits = vec![0; main_bits.len()];
            } else {
                add_bits = main_bits.clone();
            }

            for _ in 0..n {
                add_bits.insert(0, 0);
            }

            bits = self._add(bits, add_bits);
        }

        Self { bits }
    }

    pub fn and(&self, num: BigNumber) -> Self {
        let mut bits: Vec<u8> = Vec::new();
        let iter_len = if self.bits.len() > num.bits.len() {self.bits.len()} else {num.bits.len()};

        for n in 0..iter_len {
            let bit1 = self.bits.get(n).unwrap_or(&0);
            let bit2 = num.bits.get(n).unwrap_or(&0);

            if *bit1 == 1 && *bit2 == 0 || *bit1 == 0 && *bit2 == 1 || *bit1 == 0 && *bit2 == 0 {
                bits.push(0);
            } else {
                bits.push(1);
            }
        }

        Self { bits }
    }

    pub fn or(&self, num: BigNumber) -> Self {
        let mut bits: Vec<u8> = Vec::new();
        let iter_len = if self.bits.len() > num.bits.len() {self.bits.len()} else {num.bits.len()};

        for n in 0..iter_len {
            let bit1 = self.bits.get(n).unwrap_or(&0);
            let bit2 = num.bits.get(n).unwrap_or(&0);

            if *bit1 == 1 && *bit2 == 0 || *bit1 == 0 && *bit2 == 1 || *bit1 == 1 && *bit2 == 1 {
                bits.push(1);
            } else {
                bits.push(0);
            }
        }

        Self { bits }
    }

    pub fn xor(&self, num: BigNumber) -> Self {
        let mut bits: Vec<u8> = Vec::new();
        let iter_len = if self.bits.len() > num.bits.len() {self.bits.len()} else {num.bits.len()};

        for n in 0..iter_len {
            let bit1 = self.bits.get(n).unwrap_or(&0);
            let bit2 = num.bits.get(n).unwrap_or(&0);

            if *bit1 == 0 && *bit2 == 0 || *bit1 == 1 && *bit2 == 1 {
                bits.push(0);
            } else {
                bits.push(1);
            }
        }

        Self { bits }
    }

    pub fn not(&self) -> Self {
        let mut bits: Vec<u8> = Vec::new();

        for n in 0..self.bits.len() {
            let bit = self.bits.get(n).unwrap_or(&0);

            if *bit == 0 {
                bits.push(1);
            } else {
                bits.push(0);
            }
        }

        Self { bits }
    }

    pub fn shift_l(&self, num: u64) -> Self {
        let mut bits: Vec<u8> = self.bits.clone();

        for _ in 0..num {
            bits.insert(0, 0);
        }

        Self { bits }
    }

    pub fn shift_r(&self, num: u64) -> Self {
        let mut bits: Vec<u8> = Vec::new();

        if num >= self.bits.len().try_into().unwrap() {
            bits = vec![0];

            return Self { bits }
        }

        let reversed_bits = self._reverse_bits();
        let bits_len = self.bits.len() - (num as usize);
       
        for n in 0..bits_len {
            dbg!(reversed_bits[n as usize]);
            bits.push(reversed_bits[n as usize]);
        }

        Self { bits: bits.iter().rev().cloned().collect() }
    }

    fn _add(&self, bits1: Vec<u8>, bits2: Vec<u8>) -> Vec<u8> {
        let mut bits: Vec<u8> = Vec::new();
        let iter_len = if self.bits.len() > bits1.len() {self.bits.len()} else {bits2.len()};

        let mut remainder: u8 = 0;
        for n in 0..iter_len {
            let bit1 = bits1.get(n).unwrap_or(&0);
            let bit2 = bits2.get(n).unwrap_or(&0);

            let sum = bit1 + bit2 + remainder;

            if sum == 0 || sum == 1 {
                bits.push(sum);
                remainder = 0;
            } else if sum == 2 {
                bits.push(0);
                remainder = 1;
            } else if sum == 3 {
                bits.push(1);
                remainder = 1;
            } else {
                panic!("Sum is more than 3")
            }
        }

        if remainder == 1 {
            bits.push(1);
        }

        bits
    }

    fn _reverse_bits(&self) -> Vec<u8> {
        self.bits.iter().rev().cloned().collect()
    }

    fn _add_dec(&self, a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
        let mut result = vec![];
        let mut carry = 0;
    
        let (a, b) = (a.clone(), b.clone());
    
        let (mut a, mut b) = if a.len() < b.len() { (b, a) } else { (a, b) };
    
        while !a.is_empty() || !b.is_empty() {
            let x = a.pop().unwrap_or(0);
            let y = b.pop().unwrap_or(0);
            let sum = x + y + carry;
            result.push(sum % 10);
            carry = sum / 10;
        }
    
        if carry > 0 {
            result.push(carry);
        }
    
        result.reverse();
        result
    }
}

fn divide_by_two(dec: &mut Vec<u8>) -> u8 {
    let mut rem = 0;
    for d in dec.iter_mut() {
        let val = rem * 10 + (*d - b'0');
        *d = val / 2 + b'0';
        rem = val % 2;
    }
    while !dec.is_empty() && dec[0] == b'0' {
        dec.remove(0);
    }
    rem
}

fn all_zero(dec: &[u8]) -> bool {
    for d in dec {
        if *d != b'0' {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_bits() {
        assert_eq!(BigNumber::from_dec("10").to_bits(), "1010");
        assert_eq!(BigNumber::from_dec("100").to_bits(), "1100100");
        assert_eq!(BigNumber::from_dec("10000").to_bits(), "10011100010000");
        assert_eq!(BigNumber::from_dec("36975474653157054774828021269746402683982340877533073654506438589893109550756").to_bits(), "101000110111111011000001000010000010100101011010101011100100110101000111100000110111110110000001001100011110111011110110001101101010100111111111011001001111000011111111000110101010010100010100111010011000001110101111111110111100110010001110000111010100100");     
        assert_eq!(BigNumber::from_dec("29057074580068875642242131450566062848035919160319378964815985283976398254876").to_bits(), "100000000111101101110001010110110001000101000111001001100101010000010110111111010000001100010011010111011011001111011101111111110111000000100100001110111111010110000000101110000110101000100101111110110110011100101101101110101110011111101100011001100011100");        
        assert_eq!(BigNumber::from_dec("7920500443340230485711763677164507885638082568214452725630423043519603097016").to_bits(), "1000110000010110110000010100110011100000011101100010000001100101010001011111100111111010010010011011000101110100101011110010011101100111011011010111110000010101111111101000101100111100110001000100101110010010000010010000010010101101100010011110110111000");        
        assert_eq!(BigNumber::from_dec("24849215562603284913598209603366166109517208715847997125055008087001896692608").to_bits(), "11011011110000001010000101100000001011101100000010110011001000001001110010101010011010000000100000111101000010000000001110001101000110111000100111011010101110011001100100111001000101111011101000000001110100010101010111010011100010111101011010101110000000");        
   
    }

    #[test]
    fn to_dec() {
        assert_eq!(BigNumber::from_dec("10").to_dec(), "10");
        assert_eq!(BigNumber::from_dec("100").to_dec(), "100");
        assert_eq!(BigNumber::from_dec("10000").to_dec(), "10000");
        assert_eq!(BigNumber::from_dec("36975474653157054774828021269746402683982340877533073654506438589893109550756").to_dec(), "36975474653157054774828021269746402683982340877533073654506438589893109550756");
        assert_eq!(BigNumber::from_dec("29057074580068875642242131450566062848035919160319378964815985283976398254876").to_dec(), "29057074580068875642242131450566062848035919160319378964815985283976398254876");
        assert_eq!(BigNumber::from_dec("7920500443340230485711763677164507885638082568214452725630423043519603097016").to_dec(), "7920500443340230485711763677164507885638082568214452725630423043519603097016");
        assert_eq!(BigNumber::from_dec("24849215562603284913598209603366166109517208715847997125055008087001896692608").to_dec(), "24849215562603284913598209603366166109517208715847997125055008087001896692608");
    }

    #[test]
    fn add() {
        assert_eq!(BigNumber::from_dec("10").add(BigNumber::from_dec("100")).to_dec(), "110");
        assert_eq!(BigNumber::from_dec("10").add(BigNumber::from_dec("1000")).to_dec(), "1010");
        assert_eq!(BigNumber::from_dec("10").add(BigNumber::from_dec("10000")).to_dec(), "10010");
        assert_eq!(BigNumber::from_dec("24849215562603284913598209603366166109517208715847997125055008087001896692608").add(BigNumber::from_dec("50928023637484727649580938471131156146596844332739972526188064849031132936923")).to_dec(), "75777239200088012563179148074497322256114053048587969651243072936033029629531");
    }

    #[test]
    fn sub() {
        assert_eq!(BigNumber::from_dec("1").sub(BigNumber::from_dec("1")).to_dec(), "0");
        assert_eq!(BigNumber::from_dec("100").sub(BigNumber::from_dec("10")).to_dec(), "90");
        assert_eq!(BigNumber::from_dec("1000").sub(BigNumber::from_dec("10")).to_dec(), "990");
        assert_eq!(BigNumber::from_dec("10000").sub(BigNumber::from_dec("10")).to_dec(), "9990");
        assert_eq!(BigNumber::from_dec("23433380516137519328100865177402731197275054200316142385946787265837220691644").sub(BigNumber::from_dec("15790992609122403962304850700720982462507065697289326364772486909900489227779")).to_dec(), "7642387907015115365796014476681748734767988503026816021174300355936731463865");
    }

    #[test]
    fn mul() {
        assert_eq!(BigNumber::from_dec("1").mul(BigNumber::from_dec("1")).to_dec(), "1");
        assert_eq!(BigNumber::from_dec("1").mul(BigNumber::from_dec("0")).to_dec(), "0");
        assert_eq!(BigNumber::from_dec("0").mul(BigNumber::from_dec("1")).to_dec(), "0");
        assert_eq!(BigNumber::from_dec("10").mul(BigNumber::from_dec("333")).to_dec(), "3330");
        assert_eq!(BigNumber::from_dec("166807296821353892614856011957272440545").mul(BigNumber::from_dec("202007170467897058509973056246940892450")).to_dec(), "33696270044280338936322427828590295140440434162902192260288683255971864385250");
    }

    #[test]
    fn and() {
        assert_eq!(BigNumber::from_dec("1").and(BigNumber::from_dec("1")).to_dec(), "1");
        assert_eq!(BigNumber::from_dec("333").and(BigNumber::from_dec("666")).to_dec(), "8");
        assert_eq!(BigNumber::from_dec("987654321").and(BigNumber::from_dec("123456789")).to_dec(), "39471121");
    }

    #[test]
    fn or() {
        assert_eq!(BigNumber::from_dec("1").or(BigNumber::from_dec("1")).to_dec(), "1");
        assert_eq!(BigNumber::from_dec("333").or(BigNumber::from_dec("666")).to_dec(), "991");
        assert_eq!(BigNumber::from_dec("987654321").or(BigNumber::from_dec("123456789")).to_dec(), "1071639989");
    }

    #[test]
    fn xor() {
        assert_eq!(BigNumber::from_dec("1").xor(BigNumber::from_dec("1")).to_dec(), "0");
        assert_eq!(BigNumber::from_dec("333").xor(BigNumber::from_dec("666")).to_dec(), "983");
        assert_eq!(BigNumber::from_dec("987654321").xor(BigNumber::from_dec("123456789")).to_dec(), "1032168868");
    }

    #[test]
    fn not() {
        assert_eq!(BigNumber::from_dec("1").not().to_dec(), "0");
        assert_eq!(BigNumber::from_dec("333").not().to_dec(), "178");
        assert_eq!(BigNumber::from_dec("987654321").not().to_dec(), "86087502");
    }

    #[test]
    fn shift_l() {
        assert_eq!(BigNumber::from_dec("1").shift_l(3).to_bits(), "1000");
        assert_eq!(BigNumber::from_dec("5").shift_l(3).to_bits(), "101000");
        assert_eq!(BigNumber::from_dec("5").shift_l(10).to_bits(), "1010000000000");
    }

    #[test]
    fn shift_r() {
        assert_eq!(BigNumber::from_dec("12").shift_r(1).to_bits(), "110");
        assert_eq!(BigNumber::from_dec("12").shift_r(3).to_bits(), "1");
        assert_eq!(BigNumber::from_dec("12").shift_r(4).to_bits(), "0");
        assert_eq!(BigNumber::from_dec("987654321").shift_r(3).to_bits(), "111010110111100110100010110");
    }
}

// 51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4 = 36975474653157054774828021269746402683982340877533073654506438589893109550756
// XOR
// 403db8ad88a3932a0b7e8189aed9eeffb8121dfac05c3512fdb396dd73f6331c = 29057074580068875642242131450566062848035919160319378964815985283976398254876
// result
// 1182d8299c0ec40ca8bf3f49362e95e4ecedaf82bfd167988972412095b13db8 = 7920500443340230485711763677164507885638082568214452725630423043519603097016

// 36f028580bb02cc8272a9a020f4200e346e276ae664e45ee80745574e2f5ab80 = 24849215562603284913598209603366166109517208715847997125055008087001896692608
// ADD
// 70983d692f648185febe6d6fa607630ae68649f7e6fc45b94680096c06e4fadb = 50928023637484727649580938471131156146596844332739972526188064849031132936923
// result
// a78865c13b14ae4e25e90771b54963ee2d68c0a64d4a8ba7c6f45ee0e9daa65b = 75777239200088012563179148074497322256114053048587969651243072936033029629531

// 33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc = 23433380516137519328100865177402731197275054200316142385946787265837220691644
// SUB
// 22e962951cb6cd2ce279ab0e2095825c141d48ef3ca9dabf253e38760b57fe03 = 15790992609122403962304850700720982462507065697289326364772486909900489227779
// result
// 10e570324e6ffdbc6b9c813dec968d9bad134bc0dbb061530934f4e59c2700b9 = 7642387907015115365796014476681748734767988503026816021174300355936731463865

// 7d7deab2affa38154326e96d350deee1 = 166807296821353892614856011957272440545
// MUL
// 97f92a75b3faf8939e8e98b96476fd22 = 202007170467897058509973056246940892450
// result
// 4a7f69b908e167eb0dc9af7bbaa5456039c38359e4de4f169ca10c44d0a416e2 = 33696270044280338936322427828590295140440434162902192260288683255971864385250
