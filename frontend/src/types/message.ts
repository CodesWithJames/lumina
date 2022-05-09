
export type Message = {
    id: string;
    text: string;
    type: MessageType;
}

export enum MessageType {
    Warning,
    Error,
    Success,
    Info,
}