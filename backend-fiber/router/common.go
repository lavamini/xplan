package router

const PAGE_SIZE_DEFAULT = 20
const PAGE_SIZE_MIN = 10
const PAGE_SIZE_MAX = 100

type Pagination struct {
	Page     int `query:"page"`
	PageSize int `query:"page_size"`
}

func ParsePagination(params *Pagination) (int, int, int) {
	page := params.Page
	if page <= 0 {
		page = 1
	}

	page_size := params.PageSize
	if page_size <= 0 {
		page_size = PAGE_SIZE_DEFAULT
	}
	if page_size < PAGE_SIZE_MIN {
		page_size = PAGE_SIZE_MIN
	}
	if page_size > PAGE_SIZE_MAX {
		page_size = PAGE_SIZE_MAX
	}

	offset := (page - 1) * page_size
	return page, page_size, offset
}
