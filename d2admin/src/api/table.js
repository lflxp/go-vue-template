import { request } from '@/api/_service.js'

export function tablelist() {
    return request({
        url: '/tablelist',
        method: 'get'
    })
}