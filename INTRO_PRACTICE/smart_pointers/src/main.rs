use std::rc::Rc;

fn reference_counting_simple(){
    let num = 10;
    let num_in_heap = Rc::new(num);

    let clone_1 = Rc::clone(&num_in_heap);
    let clone_2 = Rc::clone(&num_in_heap);
    let clone_3 = Rc::clone(&num_in_heap);

    println!("num in heap has: {} reference(s)", Rc::strong_count(&num_in_heap));
}

//Sharing Resources
struct FamilyMember {
    tv: Rc<TV>,
}

struct TV;

fn sharing_resource_rc_count(){
    fn member_start_watch_tv() -> FamilyMember{
        let tv_is_on = Rc::new(TV);
        FamilyMember{
            tv: Rc::clone(&tv_is_on),
        }
    }
    let dad = member_start_watch_tv();
    println!("How many people watching {}", Rc::strong_count(&dad.tv));
    let mom = FamilyMember { tv: Rc::clone(&dad.tv) };
    println!("How many people watching {}", Rc::strong_count(&dad.tv));

    let me = FamilyMember { tv: Rc::clone(&dad.tv) };
    println!("How many people watching {}", Rc::strong_count(&me.tv));

    drop(dad);
    drop(me);

    println!("How many people watching {}", Rc::strong_count(&mom.tv));

}


use std::cell::RefCell;

fn ref_cell_simple(){
    let num = 10;
    let data = RefCell::new(num);

    let data_ref = data.borrow();
    println!("data in RefCell: {}", *data_ref);

    drop(data_ref);
    println!("Data: {:?}", data);

    let mut data_mut = data.borrow_mut();
    *data_mut += 1;
    println!("Data: {}", data_mut);
}

fn main(){
    reference_counting_simple();
    sharing_resource_rc_count();
    ref_cell_simple();
}