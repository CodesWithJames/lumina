import type { RequestEvent } from '@sveltejs/kit'
import type { MaybePromise } from '@sveltejs/kit/types/private'

export function getSession(event: RequestEvent): MaybePromise<App.Session> {
    return {
        user: {
            _id: '123',
            email: 'john@example.com',
            first_name: 'John',
            last_name: 'Doe',
        },
        messages: [],
        auth_token: undefined,
        BACKENDAPI_HOST: "https://api.albert.lumina.earth",
    }
}