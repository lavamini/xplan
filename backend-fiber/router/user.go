package router

import (
	"backend-fiber/database"
	"database/sql"
	"fmt"
	"strings"
	"time"

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
	user.Post("/signup", u.signup)
	user.Get("/users", u.users)
}

// user form
type UserForm struct {
	Name     string `json:"name"`
	Password string `json:"password"`
}

// signin
func (u *User) signin(c *fiber.Ctx) error {
	userForm := new(UserForm)

	if err := c.BodyParser(userForm); err != nil {
		return c.JSON(fiber.Map{
			"code": 1,
			"msg":  "parameters missing",
		})
	}

	name := strings.TrimSpace(userForm.Name)
	password := strings.TrimSpace(userForm.Password)

	if name == "" || password == "" {
		return c.JSON(fiber.Map{
			"code": 1,
			"msg":  "parameters missing",
		})
	}

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

// signup
func (u *User) signup(c *fiber.Ctx) error {
	userForm := new(UserForm)

	if err := c.BodyParser(userForm); err != nil {
		return c.JSON(fiber.Map{
			"code": 1,
			"msg":  "parameters missing",
		})
	}

	name := strings.TrimSpace(userForm.Name)
	password := strings.TrimSpace(userForm.Password)

	if name == "" || password == "" {
		return c.JSON(fiber.Map{
			"code": 1,
			"msg":  "parameters missing",
		})
	}

	rows, err := database.Db.Query("SELECT id FROM user WHERE name = ?", name)
	if err != nil {
		fmt.Printf("select id error: %s\n", err.Error())
		return c.JSON(fiber.Map{
			"code": 1,
			"msg":  "signup failed",
		})
	}

	if rows.Next() {
		return c.JSON(fiber.Map{
			"code": 1,
			"msg":  "name already exist",
		})
	} else {
		bytes, err := bcrypt.GenerateFromPassword([]byte(password), 10)
		if err != nil {
			return c.JSON(fiber.Map{
				"code": 1,
				"msg":  "signup failed",
			})
		} else {
			password_pash := string(bytes)
			created_at := time.Now().Format("2006-01-02 15:04:05")
			updated_at := created_at

			_, err := database.Db.Exec("INSERT INTO user(name, password_hash, created_at, updated_at) VALUES(?, ?, ?, ?)", name, password_pash, created_at, updated_at)
			if err != nil {
				return c.JSON(fiber.Map{
					"code": 1,
					"msg":  "signup failed",
				})
			} else {
				// signup success
				return c.JSON(fiber.Map{
					"code": 0,
					"msg":  "signup success",
				})
			}
		}
	}
}

// user entity
type UserEntity struct {
	Id        uint64 `json:"id"`
	Name      string `json:"name"`
	CreatedAt string `json:"created_at"`
	UpdatedAt string `json:"updated_at"`
}

// users
func (u *User) users(c *fiber.Ctx) error {
	rows, err := database.Db.Query("SELECT id, name, created_at, updated_at FROM user")
	if err != nil {
		fmt.Printf("select users error: %s\n", err.Error())
		return c.JSON(fiber.Map{
			"code": 1,
			"msg":  "select users failed",
		})
	}

	var userEntityArray []UserEntity

	for rows.Next() {
		var id uint64
		var name string
		var created_at, updated_at sql.NullTime

		rows.Scan(&id, &name, &created_at, &updated_at)

		userEntity := UserEntity{
			Id:        id,
			Name:      name,
			CreatedAt: created_at.Time.Format("2006-01-02 15:04:05"),
			UpdatedAt: updated_at.Time.Format("2006-01-02 15:04:05"),
		}

		userEntityArray = append(userEntityArray, userEntity)
	}

	return c.JSON(fiber.Map{
		"code": 1,
		"data": userEntityArray,
		"msg":  "success",
	})
}
