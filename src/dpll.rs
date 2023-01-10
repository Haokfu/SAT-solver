pub mod cnf_formula;
use cnf_formula::*;

pub fn find_propogatable(f:& Formula) -> Option<(Variable,bool)> {
    let mut b = false;
    let mut var: char;
    let len = f.len();
    let mut i: usize = 0;
    while i < len{
        if f[i].len() == 1{
            match f[i][0]{
                Atom::Base(a) =>{
                    b = true;
                    var = a;
                }
                Atom::Not(a) =>{
                    b = false;
                    var = a;
                }
            }
            return Some((var,b));
        }
        i = i + 1;
    }
    return None
}

pub fn hasBaseClause(c:& Clause,v:Variable) -> Vec<usize>{
    let mut indeices: Vec<usize> = vec![];
    let mut i: usize = 0;
    for a in c {
        match a {
            Atom::Base(vp) => 
                if *vp == v {
                    indeices.push(i);
                }
            Atom::Not(vp) => 
                if *vp == v {
                    i = i + 1;
                    continue;
                }
        }
        i = i+1;
    }
    return indeices;
}

pub fn hasNotClause(c:& Clause,v:Variable) -> Vec<usize>{
    let mut indeices: Vec<usize> = vec![];
    let mut i: usize = 0;
    for a in c {
        match a {
            Atom::Base(vp) => 
                if *vp == v {
                    i = i + 1;
                    continue;
                }
            Atom::Not(vp) => 
                if *vp == v {
                    indeices.push(i);
                }
        }
        i = i + 1;
    }
    return indeices;
}

pub fn propogate_unit(f:& mut Formula,v:Variable,b:bool) {
    if b == true{
        
        let f1 = f.clone();
        let mut index: usize = 0;
        let len = f1.len();
        
        for a in f1{
            let mut vecBase = hasBaseClause(&a, v);
            let mut vecNot = hasNotClause(&a, v);
            if vecBase.len() > 0{
                f.remove(index);
                continue;
            }
            else if vecNot.len() > 0{
                for j in vecNot{
                    f[index].remove(j);
                }
            }
            index = index + 1;
        }
    }
    else{
        
        let f1 = f.clone();
        let mut index: usize = 0;
        let len = f1.len();
        let mut i: usize = 0;
        for a in f1{
            let mut vecBase = hasBaseClause(&a, v);
            let mut vecNot = hasNotClause(&a, v);
            if vecNot.len() > 0{
                f.remove(index);
                continue;
            }
            else if vecBase.len() > 0{
                for j in vecBase{
                    f[index].remove(j);
                }
            }
            index = index + 1;
        }
    }
}

pub fn find_pure_var(f:& Formula) -> Option<Variable> {
    let mut var: Vec<Variable> = get_vars(f);
    for a in var{
        if is_pure(f, a){
            return Some(a);
        }
    }
    return None;
}

pub fn assign_pure_var(f: & mut Formula, v: Variable) {
    let mut i: usize = 0;
    let f1 = f.clone();
    for a in f1{
        if has_var_clause(&a, v){
            f.remove(i);
            continue;
        }
        i += 1;
    }
}

pub fn unit_propogate(f:& mut Formula) {
    match find_propogatable(f) {
        Option::None => return,
        Option::Some((v,b)) => {
            propogate_unit(f, v, b);
            unit_propogate(f)
        }
    }
}

pub fn assign_pure_vars(f:& mut Formula) {
    match find_pure_var(f) {
        Option::None => return,
        Option::Some(v) => {
            assign_pure_var(f,v);
            assign_pure_vars(f); 
        }
    }
}

pub fn dpll(f:& mut Formula) -> bool {
    //let mut new_f = f.clone();
    let null: Vec<Atom> = vec![];
    unit_propogate( f);
    assign_pure_vars( f);
    if f.len() == 0{
        return true;
    }
    else if f.contains(&null){
        return false;
    }
    else{
        let vars = get_vars(&f);
        let mut var = vars[0];
        let mut f1 = f.clone();
        let mut c1 = vec![Atom::Base(var)];
        let mut c2 = vec![Atom::Not(var)];
        f1.push(c1);
        f.push(c2);
        let b1 = dpll(f);
        let b2 = dpll(& mut f1);
        return b1 || b2;
    }
}