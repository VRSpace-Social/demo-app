// fetch.ts
import { invoke } from "@tauri-apps/api/tauri";

let regex = /^(\{.*\}|\[.*\])$/;

async function fetchData(url: string, setUserAgent?: boolean, exceptJson?: boolean): Promise<string | undefined>{
    if(!setUserAgent) setUserAgent = false;
    const response: string = await invoke('fetch_url', { url, setUserAgent });
    if(exceptJson && response && regex.test(response)) return JSON.parse(response);
    if(response && typeof response === 'string') 
        return response;
    return undefined;
}

export { fetchData };