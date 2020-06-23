use std::char;

/// add two arbitrarily large numbers
fn add(a: &Vec<char>, b: &Vec<char>) -> Vec<char> {
    let a_neg = is_negative(a);
    let b_neg = is_negative(b);

    let a_abs = abs(a);
    let b_abs = abs(b);

    if a_neg {
        if b_neg {
            // -a + -b = -(a - b)
            negate(&subtract_pos(&a_abs, &b_abs))
        } else {
            // -a + b = b - a
            subtract_pos(&b_abs, &a_abs)
        }
    } else {
        if b_neg {
            // a + (-b) = a - b
            subtract_pos(&a_abs, &b_abs)
        } else {
            // a + b
            add_pos(&a_abs, &b_abs)
        }
    }
}

/// computes a + b, where a, b >= 0
fn add_pos(a: &Vec<char>, b: &Vec<char>) -> Vec<char> {
    let mut answer = Vec::new();

    let long: &Vec<char>;
    let short: &Vec<char>;

    if a.len() >= b.len() {
        long = a;
        short = b;
    } else {
        long = b;
        short = a;
    };

    let mut long_idx = long.len();
    let mut short_idx = short.len();

    let mut carry: u32 = 0;
    let mut sum: u32;

    let mut long_val: u32;
    let mut short_val: u32;

    while long_idx > 0 {
        long_val = long[long_idx - 1].to_digit(10).unwrap();

        if short_idx > 0 {
            short_val = short[short_idx - 1].to_digit(10).unwrap();
        } else {
            short_val = 0;
        }

        sum = (long_val + short_val + carry) % 10;
        carry = (long_val + short_val + carry) / 10;

        answer.push(char::from_digit(sum, 10).unwrap());

        long_idx = if long_idx > 0 { long_idx - 1 } else { 0 };
        short_idx = if short_idx > 0 { short_idx - 1 } else { 0 };
    }

    if carry != 0 {
        answer.push(char::from_digit(carry, 10).unwrap());
    }

    answer.reverse();

    answer
}

/// get the magnitude of a number
fn abs(a: &Vec<char>) -> Vec<char> {
    return if is_negative(a) {
        a[1..].to_vec().to_owned()
    } else {
        a.clone()
    }
}

/// negate a number
fn negate(a: &Vec<char>) -> Vec<char> {
    return if is_negative(a) {
        abs(a)
    } else {
        let mut v = vec!['-'];
        v.extend(a);
        v
    }
}

/// check if a number has a unary minus
fn is_negative(a: &Vec<char>) -> bool {
    a[0] == '-'
}

/// left pad with zeros
fn pad(a: &Vec<char>, len: usize) -> Vec<char> {
    if a.len() > len {
        panic!("Trying to pad to a shorter length")
    } else if a.len() == len {
        return a.clone().to_owned();
    } else {
        let diff = len - a.len();

        let mut padded = vec!['0'; diff];

        padded.extend(a);

        return padded;
    }
}

/// remove preceding zeros
fn trim(a: &Vec<char>) ->  Vec<char> {
    let mut trimmed;

    let is_neg = is_negative(a);

    let a_abs = abs(a);

    if a_abs.len() == 1 {
        if a_abs[0] == '0' {
            return vec!['0'];
        }

        trimmed = a_abs
    } else {
        if let Some(first_nonzero) = a_abs.iter().position(|&d| d != '0') {
            trimmed = a_abs[first_nonzero..].to_vec().to_owned();
        } else {
            return vec!['0'];
        }
    }

    if is_neg {
        trimmed = negate(&trimmed);
    }

    trimmed
}

/// return true if a >= b
fn larger(a: &Vec<char>, b: &Vec<char>) -> bool {
    let a_neg = is_negative(a);
    let b_neg = is_negative(b);

    if !a_neg && b_neg {
        return true;
    } else if a_neg && !b_neg {
        return false;
    }

    let abs_a = abs(a);
    let abs_b = abs(b);

    let mut abs_a_larger;

    if abs_a.len() > abs_b.len() {
        abs_a_larger = true;
    } else if abs_b.len() > abs_a.len() {
        abs_a_larger = false;
    } else {
        abs_a_larger = true;

        let mut i = 0;

        while i < abs_a.len() {
            let a_val = abs_a[i].to_digit(10).unwrap();
            let b_val = abs_b[i].to_digit(10).unwrap();

            if a_val > b_val {
                abs_a_larger = true;
                break;
            } else if b_val > a_val {
                abs_a_larger = false;
                break;
            }

            i = i + 1;
        }
    }

    return if a_neg {
        !abs_a_larger
    } else {
        abs_a_larger
    }
}

/// calculate the difference of two arbitrary large numbers
fn subtract(a: &Vec<char>, b: &Vec<char>) -> Vec<char> {
    let a_neg = is_negative(a);
    let b_neg = is_negative(b);

    let a_abs = abs(a);
    let b_abs = abs(b);

    if a_neg {
        if b_neg {
            // -a - -b = -a + b = b - a
            subtract_pos(&b_abs, &a_abs)
        } else {
            // -a - b = -(a + b)
            negate(&add_pos(&a_abs, &b_abs))
        }
    } else {
        if b_neg {
            // a - (-b) = a + b
            add_pos(&a_abs, &b_abs)
        } else {
            // a - b
            subtract_pos(&a_abs, &b_abs)
        }
    }
}

/// computes a - b, where a >= b and a, b >= 0
fn subtract_pos(aa: &Vec<char>, bb: &Vec<char>) -> Vec<char> {
    let mut answer = Vec::new();

    let a;
    let b;
    let should_negate;

    if larger(aa, bb) {
        a = aa;
        b = bb;
        should_negate = false
    } else {
        a = bb;
        b = aa;
        should_negate = true;
    }

    let mut a_idx = a.len();
    let mut b_idx = b.len();

    let mut difference: u32;

    let mut a_val: u32;
    let mut b_val: u32;

    let mut a_mut = a.clone();
    while a_idx > 0 {
        a_val = a_mut[a_idx - 1].to_digit(10).unwrap();

        if b_idx > 0 {
            b_val = b[b_idx - 1].to_digit(10).unwrap();
        } else {
            b_val = 0;
        }

        difference = if a_val >= b_val {
            a_val - b_val
        } else {
            // perform borrow
            let mut i = a_idx - 2;

            loop {
                let lender = a_mut[i].to_digit(10).unwrap();

                if lender > 0 {
                    a_mut[i] = char::from_digit(lender - 1, 10).unwrap();
                    break;
                } else {
                    a_mut[i] = char::from_digit(9, 10).unwrap();
                }

                i = i - 1;
            }

            (a_val + 10) - b_val
        };

        a_idx = if a_idx > 0 { a_idx - 1 } else { 0 };
        b_idx = if b_idx > 0 { b_idx - 1 } else { 0 };

        if a_idx > 0 || difference != 0 || answer.len() == 0 {
            answer.push(char::from_digit(difference, 10).unwrap());
        }
    }

    if should_negate {
        answer.push('-');
    }

    answer.reverse();

    trim(&answer)
}

fn multiply(a: &Vec<char>, b: &Vec<char>) -> Vec<char> {
    // karatsuba only works for greater than 4 digits
    if a.len() < 4 || b.len() < 4 {
        let a_int = a.into_iter().collect::<String>().parse::<i32>().unwrap();
        let b_int = b.into_iter().collect::<String>().parse::<i32>().unwrap();

        return (a_int * b_int).to_string().chars().collect();
    }

    let sign_diff = is_negative(a) != is_negative(b);

    let mut a_abs = abs(a);
    let mut b_abs = abs(b);

    if a_abs.len() < b_abs.len() {
        a_abs = pad(&a_abs, b_abs.len());
    } else if b_abs.len() < a_abs.len() {
        b_abs = pad(&b_abs, a_abs.len());
    }

    let mut product = karatsuba(&a_abs, &b_abs);

    if sign_diff {
        product = negate(&product);
    }

    product
}

fn karatsuba(a: &Vec<char>, b: &Vec<char>) -> Vec<char> {
    if a.len() == 1 || b.len() == 1 {
        let a_val: u32 = a.iter().collect::<String>().parse().unwrap();
        let b_val: u32 = b.iter().collect::<String>().parse().unwrap();

        return (a_val * b_val).to_string().chars().collect::<Vec<char>>();
    }

    let mid = (std::cmp::min(a.len(), b.len()) as f32 / 2.0).ceil() as usize;
    let shift = (std::cmp::min(a.len(), b.len()) as f32 / 2.0).floor() as usize;

    let ah = trim(&a[..mid].to_vec());
    let al= trim(&a[mid..].to_vec());

    let bh= trim(&b[..mid].to_vec());
    let bl = trim(&b[mid..].to_vec());

    let z0 = multiply(&al, &bl);
    let z1 = multiply(&add(&al, &ah), &add(&bl, &bh));

    let mut z2 = multiply(&ah, &bh);
    let mut z3 = subtract(&subtract(&z1, &z2), &z0);

    z2.extend(vec!['0'; 2 * shift]);
    z3.extend(vec!['0'; shift]);

    add(&add(&z2, &z3), &z0)
}

/// converts an array of chars to a string
fn v2s(a: &Vec<char>) -> String {
    a.into_iter().collect::<String>()
}

fn main() {
    let b = "3141592653589793238462643383279502884197169399375105820974944592";
    let a = "2718281828459045235360287471352662497757247093699959574966967627";

    println!("{} x {} = {}", a, b, v2s(&multiply(&a.chars().collect(), &b.chars().collect())));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_add_test(a: i128, b: i128) {
        let sum = a + b;

        println!("[run_add_test] {:?} + {:?} = {}", a, b, sum);

        assert_eq!(add(&a.to_string().chars().collect(), &b.to_string().chars().collect()),
                   sum.to_string().chars().collect::<Vec<char>>());
    }

    #[test]
    fn test_add() {
        run_add_test(-1, 2);
        run_add_test(0, 0);
        run_add_test(123, 345);
        run_add_test(999, 2);
        run_add_test(0, -99);
        run_add_test(123123123123, 5345345345);
        run_add_test(314159265358979323840974944592, 5345345345);
        run_add_test(31415926535897944592, -065518096806127861769640336);
    }

    #[test]
    fn test_add_long() {
        let a = "3141592653589793238462643383279502884197169399375105820974944592";
        let b = "27182818284590452353602874713526624977572470936999595749669676271";
        let c = "30324410938180245592065518096806127861769640336374701570644620863";

        assert_eq!(add(&a.chars().collect(), &b.chars().collect()), c.chars().collect::<Vec<char>>());
    }

    fn run_larger_test(a: i128, b: i128) {
        println!("[run_larger_test] {:?} >= {:?} = {}", a, b, a >= b);

        assert_eq!(larger(&a.to_string().chars().collect(), &b.to_string().chars().collect()),
                   a >= b);
    }

    #[test]
    fn test_larger() {
        run_larger_test(10, 1);
        run_larger_test(0, 1);
        run_larger_test(0, 0);
        run_larger_test(-99, 0);
        run_larger_test(-1, 0);
        run_larger_test(1, -2);
        run_larger_test(123123123, 123123122);
        run_larger_test(-123123123, -123123122);
        run_larger_test(22, 22);
        run_larger_test(43426, 85992);
        run_larger_test(33, 33);
    }

    #[test]
    fn test_trim() {
        assert_eq!(trim(&"01".chars().collect()), "1".chars().collect::<Vec<char>>());
        assert_eq!(trim(&"-01".chars().collect()), "-1".chars().collect::<Vec<char>>());
        assert_eq!(trim(&"999".chars().collect()), "999".chars().collect::<Vec<char>>());
        assert_eq!(trim(&"0".chars().collect()), "0".chars().collect::<Vec<char>>());
        assert_eq!(trim(&"-0100".chars().collect()), "-100".chars().collect::<Vec<char>>());
        assert_eq!(trim(&"00000100".chars().collect()), "100".chars().collect::<Vec<char>>());
        assert_eq!(trim(&"-0".chars().collect()), "0".chars().collect::<Vec<char>>());
    }

    fn run_subtract_test(a: i128, b: i128) {
        let difference = a - b;

        println!("[run_subtract_test] {:?} - {:?} = {}", a, b, difference);

        assert_eq!(subtract(&a.to_string().chars().collect(), &b.to_string().chars().collect()),
                   difference.to_string().chars().collect::<Vec<char>>());
    }

    #[test]
    fn test_subtract() {
        run_subtract_test(1, 1);
        run_subtract_test(1152, 1092);
        run_subtract_test(456, 123);
        run_subtract_test(88, -88912312);
        run_subtract_test(0, 2312);
        run_subtract_test(44, 1);
        run_subtract_test(-44, 8123123);
        run_subtract_test(-10001, 99);
        run_subtract_test(10000, 999);
        run_subtract_test(99, 99);
        run_subtract_test(81345400, 487254);
    }

    #[test]
    fn test_subtract_long() {
        let a = "3141592653589793238462643383279502884197169399375105820974944592";
        let b = "2718281828459045235360287471352662497757247093699959574966967627";
        let c = "423310825130748003102355911926840386439922305675146246007976965";

        assert_eq!(subtract(&a.chars().collect(), &b.chars().collect()), c.chars().collect::<Vec<char>>());
    }

    fn run_multiply_test(a: i128, b: i128) {
        let product = a * b;

        println!("[run_multiply_test] {:?} x {:?} = {}", a, b, product);

        assert_eq!(multiply(&a.to_string().chars().collect(), &b.to_string().chars().collect()),
                   product.to_string().chars().collect::<Vec<char>>());
    }

    #[test]
    fn test_multiply() {
        run_multiply_test(123, 456);
        run_multiply_test(1234, 4321);
        run_multiply_test(-1234, 4321);
        run_multiply_test(114, 48);
        run_multiply_test(9123, 1236);
        run_multiply_test(1000, 1001);
        run_multiply_test(1234, -11114321);
        run_multiply_test(1234, 12345);
        run_multiply_test(10000, 1000);
        run_multiply_test(123123123188888231, 101239999777700);
    }

    #[test]
    fn test_multiply_crazy() {
        let a = "521620569660240580381501935112533824300355876402474964732639141992726042699227967823547816360093417216412199245863150302861829745557067498385054945885869269956909272107975093029553211653449872027559602364806654991198818347977535663698074265425278625518184175746728909777727938000816470600161452491921732172147723501414419735685481613611573525521334757418494684385233239073941433345477624168625189835694855620992192221842725502542568876717904946016534668049886272327917860857843838279679766814541009538837863609506800642251252051173929848960841284886269456042419652850222106611863067442786220391949450471237137869609563643719172874677646575739624138908658326459958133904780275900994657640789512694683983525957098258226205224894077267194782684826014769909026401363944374553050682034962524517493996514314298091906592509372216964615157098583874105978859597729754989301617539284681382686838689427741559918559252459539594310499725246808459872736446958486538367362226260991246080512438843904512441365497627";
        let b = "538243720583531147711992606381334677687969597030983391307710987040859133746414428227726346594704745878477872019277152807317679077071572134447306057007334924369311383504931631284042512192565179806941135280131470130478164378851852909285452011658393419656213491434159562586586557055269049652098580338507224264829397285847831630577775606888764462482468579260395352773480304802900587607582510474709164396136267604492562742042083208566119062545433721315359584506877246029016187667952406163425225771954291629919306455377991403734043287526288896399587947572917464263574552540790914513571113694109119393251910760208252026187985318877058429725916778131496990090192116971737278476847268608490033770242429165130050051683233643503895170298939223345172201381280696501178440874519601212285993716231301711444846409038906449544400619869075485160263275052983491874078668088183385102283345085048608250393021332197155184306354550076682829493041377655279397517546139539846833936383047461199665385815384205685338621867252";
        let c = "280758996146828875763522195842643583215767352358127630549702842682442455773068499185028369194449473151215232815450592355068181970286709881343494442612497008881223418013903746901801438699842751740207016780308906280005256652244332049214970950426612939600191131791913549692513051083711829731850244678942384867608649967460831202483474009261406458859266718928356416165780017716054581997115562188852652577235370745906539915870325375295106722891438933732843821062792195247272198257925463540809491616483495289655664641270015125919412169735264282879353385412875608678335336939263130906434362193856307475828355819375105488295337202409983797038220067560580093330311556454429951852234667103725346738759298702694734343532478892192114278026132292625485723746288302018244171093442238398072927636397837311373731371487717152160659095835648436885945240942060044656994652297778011955267171995507279418330726753377241262565686971037126193074813018460370349215958037148278186453727734654446692751104045341041281441802912758492317805377833605001975427447328547827104289959453151654068673818735038827100761583862077666171369096746895312354403957478207683157478245545298640047283650916546036764017942555816445190191503267102785913231102883084746513917835373098854198036405504712578322713192310154852790126330977555252950958522288241867666378387729463795706561476541049638447766372422028927130561938142047794909392432378706488931244515217992749062423485457782739467668544384987460702439420301803018826358035045019552058828546971110289818601315876775865588185367338201558350017645365668750583207665388578650636499732492437917339050902514493622215299522004064247204229354737494781279990380074996752643545244245987519594471792509392704950670832148713025004233455265917053975210032472738201522193113293747327415407357502167098334079405530414234217980962163889551351757812194148389654836839008110842122270405889609484136618978370364484841930342005150654324760960560707010000061923537561401836985771883216517431259466062915011004";

        assert_eq!(multiply(&a.chars().collect(), &b.chars().collect()), c.chars().collect::<Vec<char>>());
    }
}