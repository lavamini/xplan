package main

import (
	"flag"
	"fmt"

	"github.com/gofiber/fiber/v2"
	"github.com/jessevdk/go-flags"
)

type Option struct {
	Port string `short:"p" long:"port" default:"3000"`
}

func main() {
	var opt Option
	_, err := flags.Parse(&opt)

	if err != nil {
		flag.PrintDefaults()
		return
	}

	port := opt.Port
	app := fiber.New(fiber.Config{
		DisableStartupMessage: true,
	})

	app.Get("/", func(c *fiber.Ctx) error {
		return c.SendString("Hello, fiber server")
	})

	fmt.Printf("⇨ fiber server listening on \x1b[32m%s\x1b[0m\n", port)
	app.Listen(":" + port)
}
