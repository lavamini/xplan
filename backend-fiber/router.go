package main

import (
	"backend-fiber/router"

	"github.com/gofiber/fiber/v2"
)

func SetupRouter(app *fiber.App) {
	router.NewIndex().SetupRouter(app)
	router.NewUser().SetupRouter(app)
	router.NewEmployee().SetupRouter(app)
}
