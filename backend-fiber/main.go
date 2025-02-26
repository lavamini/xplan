package main

import (
	"backend-fiber/config"
	"backend-fiber/database"
	"flag"
	"fmt"

	_ "github.com/go-sql-driver/mysql"
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

	// load config
	config, err := config.LoadConfig("config.toml")
	if err != nil {
		fmt.Printf("load config file error: %s\n", err.Error())
		return
	}

	// init database
	err = database.InitDatabase(*config.Db)
	if err != nil {
		fmt.Printf("init database error: %s\n", err.Error())
		return
	}

	app := fiber.New(fiber.Config{
		DisableStartupMessage: true,
	})

	// setup router
	SetupRouter(app)

	fmt.Printf("⇨ fiber server listening on \x1b[32m%s\x1b[0m\n", port)
	app.Listen(":" + port)
}
