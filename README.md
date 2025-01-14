# Human Body Representation in Rust

This Rust project defines a detailed representation of a human body, complete with various anatomical components, behaviors, and functionality. The main components are structured using structs, enums, and traits, allowing for modularity and clarity. The example also demonstrates how to simulate human behaviors and display the body's assembly details.

---

## Features

### 1. **Struct-based Anatomy**
- **Human**: Represents a human entity with attributes like name, age, gender, and body.
- **Body**: Composed of a head, hands, and legs.
- **Head**: Contains detailed components such as eyes, ears, nose, teeth, and more.
- **EntireHands**: Represents the arms and hands, including fingers.
- **Legs**: Represents the lower body, including toes.

### 2. **Enum for Behaviors**
- **BehaviorType**: Captures human behaviors categorized as:
  - Instinctual
  - Emotional
  - Social
  - Cognitive
  - Learned
  - Cultural

### 3. **Traits for Functionality**
- **HumanBehavior**: Enables performing specific behaviors based on the `BehaviorType` enum.
- **ShowBodyAssembly**: Displays detailed information about the human body's anatomy.

### 4. **Detailed Anatomy**
Every component of the body is represented by nested structs with properties capturing the count or presence of individual parts (e.g., number of eyes, ears, or fingers).

---

## Example Code
Below is a sample usage of the defined structs, enums, and traits:

```rust
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
```

---

## How to Run
1. Clone the repository or copy the code into your Rust project.
2. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
3. Compile and run the program using:
   ```bash
   cargo run
   ```

---

## Output
- **Behavior Simulation**:
  Displays behaviors performed by the human based on their type.
  
  Example:
  ```text
  Alice is performing instinctual behavior: eating, sleeping, seeking shelter.
  Alice is expressing emotions: laughing, crying, or reacting emotionally.
  ...
  ```

- **Body Assembly Details**:
  Provides detailed anatomical information.
  
  Example:
  ```text
  Body structure of Alice:
  Head: hear: 1, forehead: 1, temple: 2, eyebrows: 2, ...
  Hands: shoulders: 2, upper arms: 2, elbows: 2, ...
  Legs: upper legs: 2, hips: 2, thighs: 2, ...
  ```

---

## License
This project is open-source and available for educational and experimental use. Modify and enhance the code to explore Rust programming concepts.

Author
[\[Chieng Sisovin\]](https://github.com/sisovin)

Enjoy creating waves of pet noises! 🐾