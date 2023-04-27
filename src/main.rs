fn main() {
    println!("Hello, world!");
}

enum NodeState{
    Uninitialized, //Every node starts out as uninitialized -> it has to fetch information about the rest of the cluster before it can start, won't take action in votings 
    Follower, //If a node is initialized but not the only node in the cluster, it becomes a follower
    Candidate, // If a node is a follower but hasn't heard from the leader in a while, it becomes a candidate and starts an election to become the new leader
    Leader //If a node is the only node in the cluster, it starts as the leader
}
