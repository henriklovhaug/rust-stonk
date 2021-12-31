
export interface Stonk {
    timestamp: number;
    open: number;
    high: number;
    low: number;
    close: number;
    volume: number;
    adjclose: number;
}

export interface ApiStonkNames {
    stonk_name: string;
}
