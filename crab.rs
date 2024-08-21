use crate::color::Color;
use crate::cookbook::{Cookbook, Recipe};
use crate::diet::Diet;
use crate::prey::Prey;
use crate::reef::Reef;
use std::cell::RefCell;
use std::rc::Rc;
use std::result;

#[derive(Debug)]
pub struct Crab {
    // TODO: Add fields here (some in part 1, some in part 2)
    name: String,
    speed: u32,
    color: Color,
    diet: Diet,
    reefs: Vec<Rc<RefCell<Reef>>>,
    // reefs: Rc<RefCell<Reef>>,

}


// Do NOT implement Copy for Crab.
impl Crab {
    pub fn new(name: String, speed: u32, color: Color, diet: Diet) -> Crab {
        // unimplemented!();
        Crab {
            name,
            speed,
            color,
            diet,
            reefs: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        // unimplemented!();
        &self.name
    }

    pub fn speed(&self) -> u32 {
        // unimplemented!();
        self.speed
    }

    pub fn color(&self) -> &Color {
        // unimplemented!();
        &self.color
    }

    pub fn diet(&self) -> Diet {
        // unimplemented!();
        self.diet
    }

    // PART 2 BELOW
    // ------------

    /**
     * Have this crab discover a new reef, adding it to its list of reefs.
     */
    pub fn discover_reef(&mut self, reef: Rc<RefCell<Reef>>) {
        // unimplemented!();
        self.reefs.push(reef);
    }

    /**
     * Returns Some prey from one of the reefs this crab feeds from,
     * and the index of that reef in self.reefs if able to find Some prey
     * using the `take_prey` method of Reef.
     *
     * If `take_prey` returns None, try the next reef. Try each reef only once.
     *
     * If all reefs are empty, or this crab has no reefs, return None.
     */
    fn catch_prey(&mut self) -> Option<(Box<dyn Prey>, usize)> {
        // unimplemented!();
        for (i, reef_rc) in self.reefs.iter().enumerate() {
            let mut reef_ref = reef_rc.borrow_mut();
            if let Some(prey) = reef_ref.take_prey() {
                return Some((prey, i));
            }
        }
        None
    }

    /**
     * Releases the given prey back into the reef at the given index.
     */
    fn release_prey(&mut self, prey: Box<dyn Prey>, reef_index: usize) {
        // unimplemented!();
        if let Some(reef) = self.reefs.get(reef_index) {
            let mut reef_borrow = reef.borrow_mut();
            reef_borrow.add_prey(prey);
        }
    }

    /**
     * Have this crab go hunting.
     *
     * A crab will keep trying to catch prey until it succeeds,
     * or runs out of remaining prey to try to catch.
     *
     * You should keep track of all escaped prey in a local.
     *
     * Once you have finished hunting, release all escaped prey back into
     * the reefs from whence they came _before_ you return if prey was caught.
     *
     * Your algorithm might look something like this pseudocode. The challenge
     * in this task is not intended to be in figuring out the algorithm, but
     * in figuring out _how to write it in Rust_ using what you've learned so far.
     *
     * ```text
     *     there are no escaped prey yet
     *     prey has not been caught
     *     while prey can be caught
     *       if prey escapes
     *         mark the prey as escaped
     *         try again
     *     
     *       if prey is not edible by this crab
     *         mark the prey as escaped
     *         try again
     *       
     *       prey has been caught
     *       stop trying
     *     
     *     release each escaped prey back to its reef
     *     was prey caught?
     * ```
     *
     * Note: this pseudocode reads like a terrible poem.
     */
    pub fn hunt(&mut self) -> bool {
        // unimplemented!();
        // unimplemented!();
        let mut escaped_prey: Vec<(Box<dyn Prey>, usize)> = Vec::new();
        let mut prey_caught = false;
        while !prey_caught {
            let res= self.catch_prey();
            match res {
                Some(x) => {
                    let mut prey_to_hunt = x.0;
                    let reef_index = x.1;
                    if prey_to_hunt.try_escape(self) || prey_to_hunt.diet() != self.diet() {
                        println!("Prey escaped: {}", reef_index);
                        escaped_prey.push((prey_to_hunt, reef_index));
                    } else {
                        prey_caught = true;
                        break;
                    }
                },
                None => break
            }
        }

        for (prey, reef_index) in escaped_prey{
            self.release_prey(prey, reef_index);
            println!("Released escaped prey to reef: {}",reef_index )

        }
        return prey_caught;

    }
    /**
     * Returns Some of any recipe from the given cookbook that matches the crab's diet
     * preferences, or None if no such recipe exists.
     *
     * IMPORTANT: you will need to add lifetime parameters to this function. It is
     * up to you to figure out which ones and where. Do not make any other changes
     * to the signature.
     */
    // pub fn choose_recipe(&self, cookbook: &Cookbook) -> Option<&Recipe> {
    //     unimplemented!();

    // }

    pub fn choose_recipe<'a>(&self, cookbook: &'a Cookbook) -> Option<&'a Recipe> {
        // unimplemented!();
        for eachRecipe in cookbook.recipes(){
            if eachRecipe.diet() == self.diet{
                return Some(eachRecipe);
            }
        }
        return None;
    }

    
}
