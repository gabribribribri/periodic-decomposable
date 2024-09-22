use std::collections::HashMap;

struct Element {
    symbol: &'static str,
    name: &'static str,
    number: u32,
}

type TheMap = HashMap<char, Vec<&'static str>>;

fn elements_by_alpha_order() -> TheMap {
    let mut map: TheMap = HashMap::new();
    for letter in 'A'..='Z' {
        map.insert(letter, vec![]);
    }
    for elem in ELEMENTS {
        map.get_mut(&elem.symbol.chars().next().unwrap()).unwrap().push(elem.symbol);
    }
    map
}

fn word_to_elems(word: String, so_far: String, map: &TheMap) {
    if word.len() == 0 {
        println!("--> {}", so_far);
        return
    }
    if is_elem(&word[0..1], map) {
        word_to_elems(word[1..].to_string(), format!("{} {}", so_far, word.chars().next().unwrap().to_ascii_uppercase()), &map)
    }
    
    if word.len() >= 2 && is_elem(&word[0..2], map) {
        word_to_elems(word[2..].to_string(), format!("{} {}{}", so_far, word.chars().next().unwrap().to_ascii_uppercase(), word.chars().nth(1).unwrap()), &map)
    }
    
}

fn is_elem(s: &str, map: &TheMap) -> bool {
    assert!(s.len() >= 1);
    let mut schars = s.chars();
    let first_char_upper = schars.next().unwrap().to_ascii_uppercase();
    let s_with_first_upper = format!("{}{}", first_char_upper, schars.collect::<String>());
    map[&first_char_upper].contains(&&*s_with_first_upper)
}

fn main() {
    let map = elements_by_alpha_order();
    loop {
        let mut s = String::new();
        println!(">>> Enter a word");
        std::io::stdin().read_line(&mut s);
        s = s.trim().to_string();
        if s.chars().any(|c| !c.is_alphabetic()) {
            println!(">>> Nope. You can only enter letters");
            continue;
        }
        word_to_elems(s.to_lowercase(), String::new(), &map);    
    }

    
}

const ELEMENTS: [Element; 111] = [
    Element { symbol: "Ac" 	, name: "Actinium",    number: 	89}, 	 
    Element { symbol: "Ag" 	, name: "Silver",     number: 	47}, 	
    Element { symbol: "Al" 	, name: "Aluminium",   number: 	13}, 	 
    Element { symbol: "Am" 	, name: "Americium",   number: 	95}, 	 
    Element { symbol: "Ar" 	, name: "Argon",   number: 	18}, 	 
    Element { symbol: "As" 	, name: "Arsenic",    number: 	33}, 	 
    Element { symbol: "At" 	, name: "Astatine",    number: 	85}, 	 
    Element { symbol: "Au" 	, name: "Gold",    number: 	79}, 	
    Element { symbol: "B" 	, name: "Boron",         number: 	5}, 	 
    Element { symbol: "Ba" 	, name: "Barium",     number: 	56}, 	 
    Element { symbol: "Be" 	, name: "Beryllium",   number: 	4}, 	 
    Element { symbol: "Bh" 	, name: "Bhorium",    number: 	107}, 	 
    Element { symbol: "Bi" 	, name: "Bismuth",    number: 	83}, 	 
    Element { symbol: "Bk" 	, name: "Berkelium",   number: 	97}, 	 
    Element { symbol: "Br" 	, name: "Bromine",    number: 	35}, 	 
    Element { symbol: "C" 	, name: "Carbon",        number: 	6}, 	 
    Element { symbol: "Ca" 	, name: "Calcium",    number: 	20}, 	 
    Element { symbol: "Cd" 	, name: "Cadmium",    number: 	48}, 	 
    Element { symbol: "Ce" 	, name: "Cerium",     number: 	58}, 	 
    Element { symbol: "Cf" 	, name: "Californium",    number: 	98}, 	 
    Element { symbol: "Cl" 	, name: "Chlorine",    number: 	17}, 	 
    Element { symbol: "Cm" 	, name: "Curium",     number: 	96}, 	 
    Element { symbol: "Co" 	, name: "Cobalt",     number: 	27}, 	 
    Element { symbol: "Cr" 	, name: "Chromium",    number: 	24}, 	 
    Element { symbol: "Cs" 	, name: "Caesium",    number: 	55}, 	 
    Element { symbol: "Cu" 	, name: "Copper",     number: 	29}, 	
    Element { symbol: "Ds" 	, name: "Darmstadtium",    number: 	110}, 	 
    Element { symbol: "Db" 	, name: "Dubnium",    number: 	105}, 	 
    Element { symbol: "Dy" 	, name: "Dysprosium",     number: 	66}, 	 
    Element { symbol: "Er" 	, name: "Erbium",     number: 	68}, 	 
    Element { symbol: "Es" 	, name: "Einsteinium",    number: 	99}, 	 
    Element { symbol: "Eu" 	, name: "Europium",    number: 	63}, 	 
    Element { symbol: "F" 	, name: "Fluorine",          number: 	9}, 	 
    Element { symbol: "Fe" 	, name: "Iron",    number: 	26}, 	
    Element { symbol: "Fm" 	, name: "Fermium",    number: 	100}, 	 
    Element { symbol: "Fr" 	, name: "Francium",    number: 	87}, 	 
    Element { symbol: "Ga" 	, name: "Gallium",    number: 	31}, 	 
    Element { symbol: "Gd" 	, name: "Gadolinium",     number: 	64}, 	 
    Element { symbol: "Ge" 	, name: "Germanium",   number: 	32}, 	 
    Element { symbol: "H" 	, name: "Hydrogen",          number: 	1}, 	 
    Element { symbol: "He" 	, name: "Helium",     number: 	2}, 	 
    Element { symbol: "Hf" 	, name: "Hafnium",    number: 	72}, 	 
    Element { symbol: "Hg" 	, name: "Mercury",    number: 	80}, 	
    Element { symbol: "Ho" 	, name: "Holmium",    number: 	67}, 	 
    Element { symbol: "Hs" 	, name: "Hassium",    number: 	108}, 	 
    Element { symbol: "I" 	, name: "Iodine",        number: 	53}, 	 
    Element { symbol: "In" 	, name: "Indium",     number: 	49}, 	 
    Element { symbol: "Ir" 	, name: "Iridium",    number: 	77}, 	 
    Element { symbol: "K" 	, name: "Potassium",         number: 	19}, 	
    Element { symbol: "Kr" 	, name: "Krypton",    number: 	36}, 	 
    Element { symbol: "La" 	, name: "Lanthanum",   number: 	57}, 	 
    Element { symbol: "Li" 	, name: "Lithium",    number: 	3}, 	 
    Element { symbol: "Lr" 	, name: "Lawrencium",     number: 	103}, 	 
    Element { symbol: "Lu" 	, name: "Lutetium",    number: 	71}, 	 
    Element { symbol: "Md" 	, name: "Mendelevium",    number: 	101}, 	 
    Element { symbol: "Mg" 	, name: "Magnesium",   number: 	12}, 	 
    Element { symbol: "Mn" 	, name: "Manganese",   number: 	25}, 	 
    Element { symbol: "Mo" 	, name: "Molybdenum",     number: 	42}, 	 
    Element { symbol: "Mt" 	, name: "Meitnerium",     number: 	109}, 	 
    Element { symbol: "N" 	, name: "Nitrogen",          number: 	7}, 	 
    Element { symbol: "Na" 	, name: "Sodium",     number: 	11}, 	
    Element { symbol: "Nb" 	, name: "Niobium",    number: 	41}, 	 
    Element { symbol: "Nd" 	, name: "Neodymium",   number: 	60}, 	 
    Element { symbol: "Ne" 	, name: "Neon",    number: 	10}, 	 
    Element { symbol: "Ni" 	, name: "Nickel",     number: 	28}, 	 
    Element { symbol: "No" 	, name: "Nobelium",    number: 	102}, 	 
    Element { symbol: "Np" 	, name: "Neptunium",   number: 	93}, 	 
    Element { symbol: "O" 	, name: "Oxygen",        number: 	8}, 	 
    Element { symbol: "Os" 	, name: "Osmium",     number: 	76}, 	 
    Element { symbol: "P" 	, name: "Phosphorus",        number: 	15}, 	 
    Element { symbol: "Pa" 	, name: "Protactinium",    number: 	91}, 	 
    Element { symbol: "Pb" 	, name: "Lead",    number: 	82}, 	
    Element { symbol: "Pd" 	, name: "Palladium",   number: 	46}, 	 
    Element { symbol: "Pm" 	, name: "Promethium",     number: 	61}, 	 
    Element { symbol: "Po" 	, name: "Polonium",    number: 	84}, 	 
    Element { symbol: "Pr" 	, name: "Praseodymium",    number: 	59}, 	 
    Element { symbol: "Pt" 	, name: "Platinum",    number: 	78}, 	 
    Element { symbol: "Pu" 	, name: "Plutonium",   number: 	94}, 	 
    Element { symbol: "Ra" 	, name: "Radium",     number: 	88}, 	 
    Element { symbol: "Rb" 	, name: "Rubidium",    number: 	37}, 	 
    Element { symbol: "Re" 	, name: "Rhenium",    number: 	75}, 	 
    Element { symbol: "Rf" 	, name: "Rutherfordium",   number: 	104}, 	 
    Element { symbol: "Rg" 	, name: "Roentgenium",    number: 	111}, 	 
    Element { symbol: "Rh" 	, name: "Rhodium",    number: 	45}, 	 
    Element { symbol: "Rn" 	, name: "Radon",   number: 	86}, 	 
    Element { symbol: "Ru" 	, name: "Ruthenium",   number: 	44}, 	 
    Element { symbol: "S" 	, name: "Sulphur",       number: 	16}, 	 
    Element { symbol: "Sb" 	, name: "Antimony",    number: 	51}, 	
    Element { symbol: "Sc" 	, name: "Scandium",    number: 	21}, 	 
    Element { symbol: "Se" 	, name: "Selenium",    number: 	34}, 	 
    Element { symbol: "Sg" 	, name: "Seaborgium",     number: 	106}, 	 
    Element { symbol: "Si" 	, name: "Silicon",    number: 	14}, 	 
    Element { symbol: "Sm" 	, name: "Samarium",    number: 	62}, 	 
    Element { symbol: "Sn" 	, name: "Tin",    number: 	50}, 	
    Element { symbol: "Sr" 	, name: "Strontium",   number: 	38}, 	 
    Element { symbol: "Ta" 	, name: "Tantalum",    number: 	73}, 	 
    Element { symbol: "Tb" 	, name: "Terbium",    number: 	65}, 	 
    Element { symbol: "Tc" 	, name: "Technetium",     number: 	43}, 	 
    Element { symbol: "Te" 	, name: "Tellurium",   number: 	52}, 	 
    Element { symbol: "Th" 	, name: "Thorium",    number: 	90}, 	 
    Element { symbol: "Ti" 	, name: "Titanium",    number: 	22}, 	 
    Element { symbol: "Tl" 	, name: "Thallium",    number: 	81}, 	 
    Element { symbol: "Tm" 	, name: "Thulium",    number: 	69}, 	 
    Element { symbol: "U" 	, name: "Uranium",       number: 	92}, 	 
    Element { symbol: "V" 	, name: "Vanadium",          number: 	23}, 	 
    Element { symbol: "W" 	, name: "Tungsten",          number: 	74}, 	
    Element { symbol: "Xe" 	, name: "Xenon",   number: 	54}, 	 
    Element { symbol: "Y" 	, name: "Yttrium",       number: 	39}, 	 
    Element { symbol: "Yb" 	, name: "Ytterbium",   number: 	70}, 	 
    Element { symbol: "Zn" 	, name: "Zinc",    number: 	30}, 	 
    Element { symbol: "Zr" 	, name: "Zirconium",   number: 	40}, 	 
];