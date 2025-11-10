use std::ops::Deref;

struct Animal;

impl Animal {
    fn animal(&self) {
        println!("Animal");
    }
}

struct Lion {
    animal: Animal,
}

impl Lion {
    fn animal(&self) {
        (**self).animal();
        println!("Lion!");
    }
}

impl Deref for Lion {
    type Target = Animal;
    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}

struct Tiger;

impl Tiger {
    fn animal(&self) {
        println!("Tiger");
    }
}

struct Liger {
    lion: Lion,
}

impl Deref for Liger {
    type Target = Lion;

    fn deref(&self) -> &Self::Target {
        &self.lion
    }
}

fn test() {
    let liger = Liger {
        lion: Lion { animal: Animal },
    };
    liger.animal();
}
