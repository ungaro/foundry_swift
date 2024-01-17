import Foundation

let group = DispatchGroup()
group.enter()

print("This is swift code calling Rust function.")
Task {

    //let contractName = await get_contract_name_from_rust("0xBB9bc244D798123fDe783fCc1C72d3Bb8C189413")
    //print("We're in Swift again. Contract Name: \(contractName.origin.toString())")
    
    let pair = await uniswapv2_pair();
    group.leave()
}

group.wait()
