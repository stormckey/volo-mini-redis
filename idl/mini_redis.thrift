namespace rs mini_redis

struct SetRequest{
    1: required string key, 
    2: required string value,
    3: required bool sync
    4: optional i32 expire_time,
    5: optional i32 transaction_id,
}

struct DelRequest{
    1: required string key,
    2: required bool sync,
    3: optional i32 transaction_id,
}

struct SubsrcibeRequest{
    1: required list<string> channels,
    2: required bool block,
}

struct PublishRequest{
    1: required string channel,
    2: required string message,
}

enum SubscribeType {
    Print,
    Trap,
}

struct SubscribeResponse {
    1: required bool trap,
    2: required string message,
}

service RedisService {
    string set(1: SetRequest request),
    string get(1: string key),
    string del(1: DelRequest request),
    string ping(1: string message),
    SubscribeResponse subscribe(1: SubsrcibeRequest request),
    string publish(1: PublishRequest request),
    string multi(),
    map<string,string> register(1: string port),
    string exec(1: i32 transaction_id),
    string watch(1: string key, 2: i32 transaction_id),
}

