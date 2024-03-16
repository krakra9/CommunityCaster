import { useEffect, useState } from "react";
import { magic } from "./lib/magic.js";
import Header from "./components/Header.js";
import Loading from "./components/Loading.js";
import LoginForm from "./components/Login.js";
import Logout from "./components/Logout.js";
import UserInfo from "./components/UserInfo.js";
import Footer from "./components/Footer.js";
import MyButton from "./components/MyButton.js";

function App() {
  const [user, setUser] = useState();

  useEffect(() => {
    setUser({ loading: true });

    magic.user
      .isLoggedIn()
      .then((isLoggedIn) => {
        if (isLoggedIn) {
          magic.user.getMetadata().then((userData) => {
            setUser(userData);
          });
        } else {
          setUser({ user: null });
        }
      })
      .catch((err) => {
        console.log("Error, isLoggedIn():");
        console.error(err);
        magic.user.logout().then(console.log);
        setUser({ user: null });
      });
  }, []);

  const handleButtonClick = () => {
    alert("Transaction was made!");
  };

  return (
    <>
      <Header />
      <main>
        {user?.loading ? (
          <Loading />
        ) : user?.issuer ? (
          <div className="user-container">
            <UserInfo userInfo={user} />
            <MyButton text="Click Me" onClick={handleButtonClick} />
            <Logout setUser={setUser} />
          </div>
        ) : (
          <LoginForm setUser={setUser} />
        )}
      </main>

      <Footer />
    </>
  );
}

export default App;
