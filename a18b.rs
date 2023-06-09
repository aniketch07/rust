#[derive(PartialEq)]
    enum Position{
        Maintainance,
        Marketing,
        Manager,
        LineSupervisor,
        Kitchenstaff,
        Assemblytech
    }
    #[derive(PartialEq)]

    enum Status{
        Terminated,
        Active
    }

    struct Employee{
        position:Position,
        status:Status,
    }

    
    fn try_access(employee: &Employee) -> Result<(), String> {
       if employee.status==Status::Terminated{
            Err(String::from("Terminated"))
       }
       else{
        Ok(())
       }

       if employee.position == Position::Maintainance{
        Ok(())
       }
       else{
        Err("invalid position".to_string())
       }
    
        // match employee.position {
        //     Position::Maintainance => Ok(()),
        //     _ => Err("invalid position".to_string()),
        // }
    }
    
    fn print_access(employee: &Employee) -> Result<(), String> {
        let attempt = try_access(employee)?;
        println!("Access granted");
        Ok(())
    }

    fn main() {
        let maintainance=Employee{
            position: Position::Maintainance,
            status: Status::Active,
        };
        match print_access(&maintainance){
            Err(e)=>println!("Access Denied {}",e),
            _=>(),
        }

    }
