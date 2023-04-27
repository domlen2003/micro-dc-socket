fn main() {
    println!("Hello, world!");
}

enum NodeState {
    Uninitialized, //Every node starts out as uninitialized -> it has to fetch information about the rest of the cluster before it can start, won't take action in votings
    Follower, //If a node is initialized but not the only node in the cluster, it becomes a follower
    Candidate, // If a node is a follower but hasn't heard from the leader in a while, it becomes a candidate and starts an election to become the new leader
    Leader,    //If a node is the only node in the cluster, it starts as the leader
}

enum ConnectionState {
    // If a bot starts, it is not connected to the discord server
    NotConnected, 
    // If a bot is not connected to the discord server, it tries to connect -> many shards could be needed so this state could take a while
    Connecting {
        target_count: u64,
    }, 
    // All shards are connected to the discord server
    Connected {
        current_count: u64,
    }, 
    // If in this state the bot is gracefully rolling out a new shard count
    UpdatingShards {
        current_count: u64,
        target_count: u64,
    }, 
}




struct BotState {
    connection_state: ConnectionState,
    //TODO: add info about which instance handles which shard

}
