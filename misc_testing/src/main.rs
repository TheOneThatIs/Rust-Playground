fn main() {

    let mut A: [&str; 3] = ["Weebull", "Displate", "Honda"];
    let mut B: [&str; 3] = ["TCGPlayer", "Walmart", "eBay"];
   

    let mut intersection: Vec<&str> = Vec::new();
    let mut isSubset = true;
 
    for m in 0..A.len() {
        for n in 0..B.len() {
            if A[m] == B[n] {
                if !intersection.contains(&A[m]) {
                    intersection.push(A[m]);
                }
            }
            else if (A[m] != B[n]) && n+1 == B.len() {
                isSubset = false;
            }
        }
    }
    
    println!("Is Subset?: {}", isSubset);

    if !isSubset && intersection.len() == 0 {
        println!("Set A and B are disjointed.")
    }
    else{
        print!("Intersected values: ");
        for element in intersection {
            print!("{} ", element);
        }
    }
}