package router

import (
	"backend-fiber/database"
	"fmt"

	"github.com/gofiber/fiber/v2"
	"golang.org/x/crypto/bcrypt"
)

type User struct{}

func NewUser() *User {
	return &User{}
}

// setup router
func (u *User) SetupRouter(app *fiber.App) {
	user := app.Group("/api")
	user.Post("/signin", u.signin)
}

// signin
type Signin struct {
	Name     string `json:"name"`
	Password string `json:"password"`
}

func (u *User) signin(c *fiber.Ctx) error {
	signin := new(Signin)

	if err := c.BodyParser(signin); err != nil {
		return c.JSON(fiber.Map{
			"code": 1,
			"msg":  "Parameters missing",
		})
	}

	name := signin.Name
	password := signin.Password

	rows, err := database.Db.Query("SELECT password_hash FROM user WHERE name = ?", name)
	if err != nil {
		fmt.Printf("select password_hash error: %s\n", err.Error())
		return c.JSON(fiber.Map{
			"code": 1,
			"msg":  "signin failed",
		})
	}

	if rows.Next() {
		var password_hash string
		rows.Scan(&password_hash)

		err = bcrypt.CompareHashAndPassword([]byte(password_hash), []byte(password))
		if err != nil {
			return c.JSON(fiber.Map{
				"code": 1,
				"msg":  "name or password not correct",
			})
		} else {
			// signin success
			return c.JSON(fiber.Map{
				"code": 0,
				"msg":  "signin success",
			})
		}
	} else {
		return c.JSON(fiber.Map{
			"code": 1,
			"msg":  "name or password not correct",
		})
	}
}
