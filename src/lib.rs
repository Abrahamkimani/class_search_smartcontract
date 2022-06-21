//! This contract registers students in an online class
//!
//! The contract helps the tr to keep track of the students present
//! and absent in an online class
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::{log, near_bindgen};
#[warn(unused_imports)]

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Class {
    date: String,
    name: String,
    id: i8,
}
 
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Student {
    admission: String,
    name: String,
}
 
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Attendance {
    classes: Vector<Class>,
    students: Vector<Student>,
    // class_id , vec<student>
    attendance: LookupMap<String, Vector<Student>>,
}
 
impl Default for Attendance {
    fn default() -> Self {
        Self {
            classes: Vector::new(b"r".to_vec()),
            students: Vector::new(b"r".to_vec()),
            attendance: LookupMap::new(b"r".to_vec()),
        }
    }
}
 
#[near_bindgen]
impl Attendance {
    pub fn register_class(&mut self, name: String, date: String) {
        let class_len = self.classes.len() + 1;
        let class = Class {
            name: name,
            date: date,
            id: class_len as i8,
        };
        self.classes.push(&class);
    }
    // todo
    pub fn register_student(&mut self, name: String, admission: String) {
      //let stude_len = self.students.len() + 1;
      let student = Student {
        admission: admission,
        name: name,
      };
      self.students.push(&student); 
    }
 
    // attendance
    pub fn attend_class(&mut self, id_class: String, admission: String) {
        // check if classs id exist in hashmap
        let class_students = self.attendance.get(&id_class);
        let stude: Vec<Student> = self
            .students
            .iter()
            .filter(|f| f.admission == admission)
            .collect();
 
        match class_students {
            Some(mut x) => {
                // ensure students has not registered before
 
                // if stude.len() > 1 {
                //     log!("Critcal more than one student found");
                //     log!("cannot register attendance");
                // } else if stude.len() == 1 {
                    let one_student = stude.get(0);
                    match one_student {
                        Some(y) => {
                            x.push(&y);
                            self.attendance.insert(&id_class, &x);
                            log!("student added to class attendance");
                        }
                        None => {
                            log!("Critcal errro ....");
                        }
                    }
                // } else {
                //     log!("Critcal student not found");
                // }
            }
            None => {
                let mut students: Vector<Student> = Vector::new(b"r".to_vec());
                let one_student = stude.get(0);
                match one_student {
                    Some(y) => {
                        students.push(&y);
                        self.attendance.insert(&id_class, &students);
                        log!("student added to class attendance");
                    }
                    None => {
                        log!("Critcal errro we do not know the student");
                    }
                }
            }
        }
    }
}
 
// Tests
#[cfg(test)]
mod tests {
    use super::*;
  //  use near_sdk::MockedBlockchain;
    //use near_sdk::{testing_env, VMContext};
 
    #[test]
    fn register_clas() {
        let mut attendance = Attendance::default();
        attendance.register_class(String::from("123"), String::from("16_06_2022"));
        assert_eq!(attendance.classes.len(), 1)
    }
  
  #[test]
  fn register_student(){
        let mut attendance = Attendance::default();
        attendance.register_student(String::from("mike"), String::from("910"));
        assert_eq!(attendance.students.len(), 1)
    
    
  }
 
    #[test]
    fn attend_class() {
        let mut attendance = Attendance::default();
 
        attendance.register_class(String::from("123"), String::from("16_06_2022"));
 
        attendance.register_student(String::from("Mike"), String::from("910")); // todo add paramter
        attendance.attend_class(String::from("123"), String::from("910"));
        assert_eq!(
            attendance
                .attendance
                .get(&String::from("123"))
                .unwrap()
                .len(),
            1
        )
    }
}