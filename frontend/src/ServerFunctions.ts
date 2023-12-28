const serverURL = import.meta.env.DEV ? "http://localhost:8000" : "";

export async function fetchUserInfo(user_id: string): Promise<string> {
  const res = await fetch(`${serverURL}/api/user/info?user_id=${user_id}`);
  if (res.ok) {
    return res.json();
  } else {
    return "user not found";
  }
}
