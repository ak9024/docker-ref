package main

import (
	"github.com/gofiber/fiber/v2"
)

func main() {
	app := fiber.New()

	app.Get("/", func(c *fiber.Ctx) error {
		ip := c.IP()
		return c.JSON(struct {
			Data string `json:"data"`
		}{
			Data: ip,
		})
	})

	app.Listen(":4000")
}
