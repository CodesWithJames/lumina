import type { MaybePromise, RequestEvent } from '@sveltejs/kit/types/private'

export function getSession(event: RequestEvent): MaybePromise<App.Session> {
    return {
        messages: [],
        auth_token: undefined,
        BACKENDAPI_HOST: "https://api.albert.lumina.earth",
    }
}