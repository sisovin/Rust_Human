// Define the `Human` struct
pub struct Human {
    name: String,
    #[allow(dead_code)]
    age: u32,
    #[allow(dead_code)]
    gender: String,
    #[allow(dead_code)]
    body: Body,
}

// Define the `Body` struct
pub struct Body {
    #[allow(dead_code)]
    head: Head,
    #[allow(dead_code)]
    hands: EntireHands,
    #[allow(dead_code)]
    legs: Legs,
}

// Define the `Head` struct
pub struct Head {
    #[allow(dead_code)]
    hear: u32,
    #[allow(dead_code)]
    forehead: u32,
    #[allow(dead_code)]
    temple: u32,
    #[allow(dead_code)]
    eyebrows: u32,
    #[allow(dead_code)]
    eyelashes: u32,
    #[allow(dead_code)]
    eyes: u32,
    #[allow(dead_code)]
    ears: u32,
    #[allow(dead_code)]
    earlobes: u32,
    #[allow(dead_code)]
    nose: u32,
    #[allow(dead_code)]
    nostrils: u32,
    #[allow(dead_code)]
    cheeks: u32,
    #[allow(dead_code)]
    lips: u32,
    #[allow(dead_code)]
    mouth: u32,
    #[allow(dead_code)]
    teeth: u32,
    #[allow(dead_code)]
    tongue: u32,
    #[allow(dead_code)]
    jaw: u32,
    #[allow(dead_code)]
    chin: u32,
}

// Define the `EntireHands` struct
pub struct EntireHands {
    #[allow(dead_code)]
    shoulders: u32,
    #[allow(dead_code)]
    upper_arms: u32,
    #[allow(dead_code)]
    elbows: u32,
    #[allow(dead_code)]
    forearms: u32,
    #[allow(dead_code)]
    wrists: u32,
    #[allow(dead_code)]
    left_hand: Hand,
    #[allow(dead_code)]
    right_hand: Hand,
}

// Define the `Legs` struct
pub struct Legs {
    #[allow(dead_code)]
    upper_legs: u32,
    #[allow(dead_code)]
    hips: u32,
    #[allow(dead_code)]
    thighs: u32,    
    #[allow(dead_code)]
    knees: u32,
    #[allow(dead_code)]
    lower_legs: u32,
    #[allow(dead_code)]
    ankles: u32,    
    #[allow(dead_code)]
    left_leg: Leg,
    #[allow(dead_code)]
    right_leg: Leg,
}

// Define the `Hand` struct
pub struct Hand {
    #[allow(dead_code)]
    fingers: u32,
}

// Define the `Leg` struct
pub struct Leg {
    #[allow(dead_code)]
    toes: u32,
}

// Enum for behavior types
pub enum BehaviorType {
    Instinctual,
    Emotional,
    Social,
    Cognitive,
    Learned,
    Cultural,
}

// Trait for human behaviors
pub trait HumanBehavior {
    fn perform_behavior(&self, behavior: BehaviorType);
}

// Trait to show the real human (body assembly)
pub trait ShowBodyAssembly {
    fn display_body(&self);
}

// Implementation of the `HumanBehavior` trait for the `Human` struct
impl HumanBehavior for Human {
    fn perform_behavior(&self, behavior: BehaviorType) {
        match behavior {
            BehaviorType::Instinctual => {
                println!("{} is performing instinctual behavior: eating, sleeping, seeking shelter.", self.name);
            }
            BehaviorType::Emotional => {
                println!("{} is expressing emotions: laughing, crying, or reacting emotionally.", self.name);
            }
            BehaviorType::Social => {
                println!("{} is engaging in social activities: interacting with others, forming connections.", self.name);
            }
            BehaviorType::Cognitive => {
                println!("{} is thinking, solving problems, and making decisions.", self.name);
            }
            BehaviorType::Learned => {
                println!("{} is demonstrating a learned skill: cooking, riding a bike, etc.", self.name);
            }
            BehaviorType::Cultural => {
                println!("{} is participating in cultural activities: celebrating festivals or following traditions.", self.name);
            }
        }
    }
}

// Implementation of the `ShowBodyAssembly` trait for the `Human` struct
impl ShowBodyAssembly for Human {
    fn display_body(&self) {
        println!("Body structure of {}:", self.name);

        // Display Head details
        println!(
            "Head: hear: {}, forehead: {}, temple: {}, eyebrows: {}, eyelashes: {}, eyes: {}, ears: {}, earlobes: {}, nose: {}, nostrils: {}, cheeks: {}, lips: {}, mouth: {}, teeth: {}, tongue: {}, jaw: {}, chin: {}",
            self.body.head.hear,
            self.body.head.forehead,
            self.body.head.temple,
            self.body.head.eyebrows,
            self.body.head.eyelashes,
            self.body.head.eyes,
            self.body.head.ears,
            self.body.head.earlobes,
            self.body.head.nose,
            self.body.head.nostrils,
            self.body.head.cheeks,
            self.body.head.lips,
            self.body.head.mouth,
            self.body.head.teeth,
            self.body.head.tongue,
            self.body.head.jaw,
            self.body.head.chin,
        );

        // Display Hands details
        println!(
            "Hands: shoulders: {}, upper arms: {}, elbows: {}, forearms: {}, wrists: {}, left hand fingers: {}, right hand fingers: {}",
            self.body.hands.shoulders,
            self.body.hands.upper_arms,
            self.body.hands.elbows,
            self.body.hands.forearms,
            self.body.hands.wrists,
            self.body.hands.left_hand.fingers,
            self.body.hands.right_hand.fingers,
        );

        // Display Legs details
        println!(
            "Legs: upper legs: {}, hips: {}, thighs: {}, knees: {}, lower legs: {}, ankles: {}, left leg toes: {}, right leg toes: {}",
            self.body.legs.upper_legs,
            self.body.legs.hips,
            self.body.legs.thighs,
            self.body.legs.knees,
            self.body.legs.lower_legs,
            self.body.legs.ankles,
            self.body.legs.left_leg.toes,
            self.body.legs.right_leg.toes,
        );
    }
}

// Example usage
fn main() {
    let human = Human {
        name: String::from("Alice"),
        age: 30,
        gender: String::from("Female"),
        body: Body {
            head: Head { 
                hear: 1,
                forehead: 1,
                temple: 2,
                eyebrows: 2,
                eyelashes: 2,
                eyes: 2,
                ears: 2,
                earlobes: 2,
                nose: 1,
                nostrils: 2,
                cheeks: 2,
                lips: 1,
                mouth: 1,
                teeth: 32,
                tongue: 1,
                jaw: 1,
                chin: 1,
            },
            hands: EntireHands {
                shoulders: 2,
                upper_arms: 2,
                elbows: 2,
                forearms: 2,
                wrists: 2,
                left_hand: Hand { fingers: 5 },
                right_hand: Hand { fingers: 5 },
            },
            legs: Legs {
                upper_legs: 2,
                hips: 2,
                thighs: 2,
                knees: 2,
                lower_legs: 2,
                ankles: 2,
                left_leg: Leg { toes: 5 },
                right_leg: Leg { toes: 5 },
            },
        },
    };

    // Perform behaviors
    human.perform_behavior(BehaviorType::Instinctual);
    human.perform_behavior(BehaviorType::Emotional);
    human.perform_behavior(BehaviorType::Social);
    human.perform_behavior(BehaviorType::Cognitive);
    human.perform_behavior(BehaviorType::Learned);
    human.perform_behavior(BehaviorType::Cultural);

    // Display the body assembly
    human.display_body();
}