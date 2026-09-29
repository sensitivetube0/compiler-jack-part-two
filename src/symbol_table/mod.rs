
#[derive(Debug)]
pub struct SymbolTable{
    class_symbol_table:ClassSymbolTable,
    subroutine_symbol_table:SubroutineSymbolTable
}

#[derive(Debug)]
struct ClassSymbolTable{
    field_vars:Vec<Field>,
    static_vars:Vec<Static>,
}

#[derive(Debug)]
struct SubroutineSymbolTable{
    arguement_vars:Vec<Arguments>,
    local_vars:Vec<Local>
}

#[derive(Debug)]
struct Static{
    static_name:String,
    static_type:Type,
}


#[derive(Debug)]
struct Arguments{
    argument_name:String,
    argument_type:Type,
}


#[derive(Debug)]
struct Local{
    local_name:String,
    local_type:Type,
}

#[derive(Debug)]
struct Field{
    field_name:String,
    field_type:Type,
}

#[derive(Debug)]
enum Type{
    BuiltIn(String),
    ClassName(String)
}


impl ClassSymbolTable{


    fn reset(&mut self){
        self.field_vars = Vec::new();
        self.static_vars = Vec::new();
    }
}


impl SubroutineSymbolTable{


    fn reset(&mut self){
        self.local_vars = Vec::new();
        self.arguement_vars = Vec::new();
    }
}


impl SymbolTable{

    pub fn new() -> Self{
        Self{
            class_symbol_table:ClassSymbolTable{
                field_vars:Vec::new(),
                static_vars:Vec::new(),
            },
            subroutine_symbol_table:SubroutineSymbolTable{
                arguement_vars:Vec::new(),
                local_vars:Vec::new(),
            }
        }
    }




}
