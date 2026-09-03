const KEY = "miydisk:skip-permanent-delete-confirm-until";

function todayString() {
  const d = new Date();
  return `${d.getFullYear()}-${d.getMonth() + 1}-${d.getDate()}`;
}

export function shouldSkipConfirm() {
  return localStorage.getItem(KEY) === todayString();
}

export function skipConfirmForToday() {
  localStorage.setItem(KEY, todayString());
}