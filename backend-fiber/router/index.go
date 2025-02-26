package router

import "github.com/gofiber/fiber/v2"

type Index struct{}

func NewIndex() *Index {
	return &Index{}
}

// setup router
func (i *Index) SetupRouter(app *fiber.App) {
	index := app.Group("")
	index.Get("/", i.index)
}

// index
func (i *Index) index(c *fiber.Ctx) error {
	return c.SendString("Hello, fiber server")
}
