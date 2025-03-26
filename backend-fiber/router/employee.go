package router

import (
	"backend-fiber/database"
	"database/sql"
	"fmt"

	"github.com/gofiber/fiber/v2"
)

type Employee struct{}

func NewEmployee() *Employee {
	return &Employee{}
}

// setup router
func (u *Employee) SetupRouter(app *fiber.App) {
	employee := app.Group("/api")
	employee.Get("/employees", u.employees)
}

// employees entity
type EmployeeEntity struct {
	EmpNo     int32  `json:"emp_no"`
	BirthDate string `json:"birth_date"`
	FirstName string `json:"first_name"`
	LastName  string `json:"last_name"`
	Gender    string `json:"gender"`
	HireDate  string `json:"hire_date"`
}

// employees
func (u *Employee) employees(c *fiber.Ctx) error {
	rows, err := database.Db.Query("SELECT emp_no, birth_date, first_name, last_name, gender, hire_date FROM employee LIMIT 20000,20")
	if err != nil {
		fmt.Printf("select employees error: %s\n", err.Error())
		return c.JSON(fiber.Map{
			"code": 1,
			"msg":  "select employees failed",
		})
	}

	employeeEntityArray := []EmployeeEntity{}

	for rows.Next() {
		var emp_no int32
		var first_name, last_name, gender string
		var birth_date, hire_date sql.NullTime

		rows.Scan(&emp_no, &birth_date, &first_name, &last_name, &gender, &hire_date)

		employeeEntity := EmployeeEntity{
			EmpNo:     emp_no,
			BirthDate: birth_date.Time.Format("2006-01-02"),
			FirstName: first_name,
			LastName:  last_name,
			Gender:    gender,
			HireDate:  hire_date.Time.Format("2006-01-02"),
		}

		employeeEntityArray = append(employeeEntityArray, employeeEntity)
	}

	return c.JSON(fiber.Map{
		"code": 0,
		"data": employeeEntityArray,
		"msg":  "success",
	})
}
