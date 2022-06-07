import toml  from 'toml'
export type CoreConfig = {
    net: {
        ipv4: Array<number>,
        port: number,
    }
    room: Array<{
        roomid: number,
        channel: Array<'json'|'bincode'>
    }>
}
const IPTEST:RegExp = /^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$/;
export function ip_valid(ip:string):boolean {
    return IPTEST.test(ip);
}

export function parse(raw_toml: string):CoreConfig {
    return toml.parse(raw_toml)
}
export function dump(config:CoreConfig):string {
    let lines:Array<string> = [];
    lines.push('[net]');
    lines.push(`ipv4=[${config.net.ipv4}]`);
    lines.push(`port=${config.net.port}`);

    for(const room of config.room) {
        lines.push('[[room]]');
        lines.push(`roomid=${room.roomid}`);
        lines.push(`channel=[${room.channel.map((s)=>(`"${s}"`))}]`);
    }

    return lines.join('\n');

}