import Cookies from 'js-cookie'

// bug: 后端会锁cookie =》 CookieHTTPOnly => js can't modify 导致一直登陆不成功
const TokenKey = 'token'

export function getToken() {
  return Cookies.get(TokenKey)
}

export function setToken(token) {
  return Cookies.set(TokenKey, token)
}

export function removeToken() {
  return Cookies.remove(TokenKey)
}
