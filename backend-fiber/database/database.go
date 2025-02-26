package database

import (
	"backend-fiber/config"
	"database/sql"
	"fmt"

	"github.com/shirou/gopsutil/cpu"
)

var Db *sql.DB

func InitDatabase(config config.Db) error {
	// 获取 cpu 核心数（不包括超线程）
	numCPU, _ := cpu.Counts(false)

	// mysql
	connStr := fmt.Sprintf("%s:%s@tcp(%s:%d)/%s?loc=Local&parseTime=true",
		config.User, config.Password, config.Host, config.Port, config.Database)

	var err error
	Db, err = sql.Open("mysql", connStr)
	if err != nil {
		return err
	}
	// 设置连接池大小
	Db.SetMaxOpenConns(numCPU*2 + 1)
	return nil
}
